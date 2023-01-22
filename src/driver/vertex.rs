//! Vertex layout types

pub use screen_13_macros::Vertex;
use {super::DriverError, ash::vk, spirq::Variable, std::collections::HashMap};

/// The state of the vertex input stage of the graphics pipeline.
#[derive(Debug, Default, Clone)]
pub struct VertexInputState {
    /// List of vertex input bindings describing how input buffers will be read.
    pub vertex_binding_descriptions: Vec<vk::VertexInputBindingDescription>,
    /// List of attribute descriptions specifying formats and locations for buffer inputs.
    pub vertex_attribute_descriptions: Vec<vk::VertexInputAttributeDescription>,
}

/// Trait used to specialize an defined VertexLayouts.
pub trait VertexLayout {
    /// Creates a VertexInputState from the VertexLayout and a set of shader-variables.
    fn specialize(&self, inputs: &[Variable]) -> Result<VertexInputState, DriverError>;
}

impl VertexLayout for VertexInputState {
    #[inline]
    fn specialize(&self, _inputs: &[Variable]) -> Result<VertexInputState, DriverError> {
        Ok(self.clone())
    }
}

impl<T> VertexLayout for &[T]
where
    T: VertexLayout,
{
    #[inline]
    fn specialize(&self, inputs: &[Variable]) -> Result<VertexInputState, DriverError> {
        // TODO: check for collisions! Update binding references!
        let mut states = Vec::with_capacity(self.len());
        for vertex in self.iter() {
            states.push(vertex.specialize(inputs)?);
        }
        Ok(VertexInputState {
            vertex_binding_descriptions: states
                .clone() // TODO: avoid this clone, maybe unzip?
                .into_iter()
                .flat_map(|state| state.vertex_binding_descriptions)
                .collect(),
            vertex_attribute_descriptions: states
                .into_iter()
                .flat_map(|state| state.vertex_attribute_descriptions)
                .collect(),
        })
    }
}

impl<T, const N: usize> VertexLayout for [T; N]
where
    T: VertexLayout,
{
    #[inline]
    fn specialize(&self, inputs: &[Variable]) -> Result<VertexInputState, DriverError> {
        self.as_slice().specialize(inputs)
    }
}

impl<T> VertexLayout for Vec<T>
where
    T: VertexLayout,
{
    #[inline]
    fn specialize(&self, inputs: &[Variable]) -> Result<VertexInputState, DriverError> {
        self.as_slice().specialize(inputs)
    }
}

pub trait Vertex {
    fn layout(input_rate: vk::VertexInputRate) -> DerivedVertexLayout;
}

pub struct DerivedVertexLayout {
    pub attributes: HashMap<String, DerivedVertexAttribute>,
    pub stride: u32,
    pub input_rate: vk::VertexInputRate,
}

pub struct DerivedVertexAttribute {
    pub offset: u32,
    pub format: vk::Format,
    pub num_locations: u32,
}

impl VertexLayout for DerivedVertexLayout {
    #[inline]
    fn specialize(&self, _inputs: &[Variable]) -> Result<VertexInputState, DriverError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Vertex, VertexInputState, VertexLayout};
    use ash::vk;
    use spirq::{ty::Type, InterfaceLocation, Variable};

    #[test]
    fn vertex_input() {
        let state = VertexInputState {
            vertex_binding_descriptions: vec![vk::VertexInputBindingDescription {
                binding: 0,
                stride: 0,
                input_rate: vk::VertexInputRate::VERTEX,
            }],
            vertex_attribute_descriptions: vec![vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32_SFLOAT,
                offset: 0,
            }],
        };
        let inputs = [Variable::Input {
            name: Some("name".to_owned()),
            location: InterfaceLocation::new(0, 0),
            ty: Type::Scalar(spirq::ty::ScalarType::Float(1)),
        }];
        let output = [state.clone(), state].specialize(&inputs).unwrap();
        // Usage in GraphicPipelineInfo would look something like `VertexInput`:
        // ```
        //     VertexLayout::Explicit([state.clone(), state]),
        // ```
        // GraphicPipeline create will require some validation logic to make sure layout makes at
        // least sense...
        assert_eq!(output.vertex_binding_descriptions.len(), 2);
        assert_eq!(output.vertex_attribute_descriptions.len(), 2);
    }

    #[test]
    fn derive_vertex() {
        #[repr(C)]
        #[derive(Vertex)]
        struct MyVertex {
            #[format(R16G16B16_SNORM)]
            normal: [i32; 3],
            #[name("in_proj", "cam_proj")]
            #[format(R32G32B32A32_SFLOAT, 4)]
            proj: [f32; 16],
        }
        let output = MyVertex::layout(vk::VertexInputRate::VERTEX);
        assert_eq!(output.attributes.len(), 3);
    }
}
