use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;

use ash::vk;
use shaderc::{CompileOptions, Compiler, IncludeType, ResolvedInclude, ShaderKind};

pub fn compile_spv_u32_data(path: PathBuf, stage_flags: vk::ShaderStageFlags) -> Vec<u32> {
    let source = fs::read_to_string(path.as_path()).expect("Couldn't read shader");

    let compiler = Compiler::new().unwrap();
    let mut options = CompileOptions::new().unwrap();
    options.set_generate_debug_info();
    let origin_path = path.clone();
    options.set_include_callback(
        move |requested_source, include_type, origin_source, recursion_depth| {
            get_sharerc_include(
                requested_source,
                include_type,
                origin_source,
                recursion_depth,
                origin_path.parent().unwrap(),
            )
        },
    );
    let sc_stage = get_shaderc_stage(&stage_flags).unwrap();
    let code = compiler
        .compile_into_spirv(
            &source,
            sc_stage,
            path.file_name().unwrap().to_str().unwrap(),
            "main",
            Some(&options),
        )
        .unwrap();

    code.as_binary().to_vec()
}

fn get_sharerc_include(
    requested_source: &str,
    _include_type: IncludeType,
    _origin_source: &str,
    _recursion_depth: usize,
    origin_dir: &Path,
) -> Result<ResolvedInclude, String> {
    //TODO: finish implementation
    let resolved_file = origin_dir.join(requested_source);
    let resolved_name = resolved_file
        // .file_name()
        // .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    //println!("Including: {}", resolved_name);
    let error_msg = format!("Failed to open {}.", resolved_file.to_str().unwrap());
    let content = fs::read_to_string(resolved_file.as_path()).expect(&error_msg);
    Ok(ResolvedInclude {
        resolved_name,
        content,
    })
}

pub fn get_shaderc_stage(stage: &vk::ShaderStageFlags) -> Option<ShaderKind> {
    if *stage == vk::ShaderStageFlags::VERTEX {
        return Some(ShaderKind::Vertex);
    } else if *stage == vk::ShaderStageFlags::FRAGMENT {
        return Some(ShaderKind::Fragment);
    } else if *stage == vk::ShaderStageFlags::COMPUTE {
        return Some(ShaderKind::Compute);
    } else if *stage == vk::ShaderStageFlags::TESSELLATION_CONTROL {
        return Some(ShaderKind::TessControl);
    } else if *stage == vk::ShaderStageFlags::TESSELLATION_EVALUATION {
        return Some(ShaderKind::TessEvaluation);
    } else if *stage == vk::ShaderStageFlags::GEOMETRY {
        return Some(ShaderKind::Geometry);
    } else if *stage == vk::ShaderStageFlags::RAYGEN_NV {
        return Some(ShaderKind::RayGeneration);
    } else if *stage == vk::ShaderStageFlags::ANY_HIT_NV {
        return Some(ShaderKind::AnyHit);
    } else if *stage == vk::ShaderStageFlags::CLOSEST_HIT_NV {
        return Some(ShaderKind::ClosestHit);
    } else if *stage == vk::ShaderStageFlags::MISS_NV {
        return Some(ShaderKind::Miss);
    } else if *stage == vk::ShaderStageFlags::INTERSECTION_NV {
        return Some(ShaderKind::Intersection);
    }
    None
}
