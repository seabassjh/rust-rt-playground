use std::path::PathBuf;

use shaders::compile_spv_u32_data;

mod shaders;

use {bytemuck::cast_slice, screen_13::prelude::*, std::sync::Arc};

// A Vulkan triangle using a graphic pipeline, vertex/fragment shaders, and index/vertex buffers.
fn main() -> Result<(), DisplayError> {
    pretty_env_logger::init();

    let event_loop = EventLoop::new().ray_tracing(true).build()?;
    let mut _cache = HashPool::new(&event_loop.device);

    // let &PhysicalDeviceRayTracePipelineProperties {
    //     shader_group_base_alignment,
    //     shader_group_handle_alignment,
    //     shader_group_handle_size,
    //     ..
    // } = event_loop
    //     .device
    //     .ray_tracing_pipeline_properties
    //     .as_ref()
    //     .unwrap();
    // let ray_trace_pipeline = create_ray_trace_pipeline(&event_loop.device)?;

    let triangle_pipeline = event_loop.new_graphic_pipeline(
        GraphicPipelineInfo::default(),
        [
            Shader::new_vertex(compile_spv_u32_data(
                PathBuf::from("./assets/shaders/triangle.vert"),
                vk::ShaderStageFlags::VERTEX,
            )),
            Shader::new_fragment(compile_spv_u32_data(
                PathBuf::from("./assets/shaders/triangle.frag"),
                vk::ShaderStageFlags::FRAGMENT,
            )),
        ],
    );

    let index_buf = Arc::new(Buffer::create_from_slice(
        &event_loop.device,
        vk::BufferUsageFlags::INDEX_BUFFER,
        cast_slice(&[0u16, 1, 2]),
    )?);

    let vertex_buf = Arc::new(Buffer::create_from_slice(
        &event_loop.device,
        vk::BufferUsageFlags::VERTEX_BUFFER,
        cast_slice(&[
            1.0f32, 1.0, 0.0, // v1
            1.0, 0.0, 0.0, // red
            0.0, -1.0, 0.0, // v2
            0.0, 1.0, 0.0, // green
            -1.0, 1.0, 0.0, // v3
            0.0, 0.0, 1.0, // blue
        ]),
    )?);

    event_loop.run(|frame| {
        let index_node = frame.render_graph.bind_node(&index_buf);
        let vertex_node = frame.render_graph.bind_node(&vertex_buf);

        frame
            .render_graph
            .begin_pass("Triangle Example")
            .bind_pipeline(&triangle_pipeline)
            .access_node(index_node, AccessType::IndexBuffer)
            .access_node(vertex_node, AccessType::VertexBuffer)
            .clear_color(0)
            .store_color(0, frame.swapchain_image)
            .record_subpass(move |subpass| {
                subpass.bind_index_buffer(index_node, vk::IndexType::UINT16);
                subpass.bind_vertex_buffer(vertex_node);
                subpass.draw_indexed(3, 1, 0, 0, 0);
            });
    })
}

// fn create_ray_trace_pipeline(device: &Arc<Device>) -> Result<Arc<RayTracePipeline>, DriverError> {
//     Ok(Arc::new(RayTracePipeline::create(
//         device,
//         RayTracePipelineInfo::new()
//             .max_ray_recursion_depth(1)
//             .build(),
//         [
//             Shader::new_ray_gen(SHADER_RAY_GEN),
//             Shader::new_closest_hit(SHADER_CLOSEST_HIT),
//             Shader::new_miss(SHADER_MISS),
//             Shader::new_miss(SHADER_SHADOW_MISS),
//         ],
//         [
//             RayTraceShaderGroup::new_general(0),
//             RayTraceShaderGroup::new_triangles(1, None),
//             RayTraceShaderGroup::new_general(2),
//             RayTraceShaderGroup::new_general(3),
//         ],
//     )?))
// }
