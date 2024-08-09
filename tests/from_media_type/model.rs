use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_additive_manufacturing_format() {
    let fmt = FileFormat::from_media_type("application/x-amf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AdditiveManufacturingFormat)), "{:?} does not contain {}", fmt, FileFormat::AdditiveManufacturingFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_autocad_drawing() {
    let fmt = FileFormat::from_media_type("application/x-dwg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AutocadDrawing)), "{:?} does not contain {}", fmt, FileFormat::AutocadDrawing);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_autodesk123d(){
    let fmt = FileFormat::from_media_type("model/x-123dx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Autodesk123d)), "{:?} does not contain {}", fmt, FileFormat::Autodesk123d);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_autodesk_alias() {
    let fmt = FileFormat::from_media_type("model/x-wire");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AutodeskAlias)), "{:?} does not contain {}", fmt, FileFormat::AutodeskAlias);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_autodesk_inventor_assembly() {
    let fmt = FileFormat::from_media_type("model/x-iam");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AutodeskInventorAssembly)), "{:?} does not contain {}", fmt, FileFormat::AutodeskInventorAssembly);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_autodesk_inventor_drawing() {
    let fmt = FileFormat::from_media_type("model/x-idw");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AutodeskInventorDrawing)), "{:?} does not contain {}", fmt, FileFormat::AutodeskInventorDrawing);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_autodesk_inventor_part() {
    let fmt = FileFormat::from_media_type("model/x-ipt");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AutodeskInventorPart)), "{:?} does not contain {}", fmt, FileFormat::AutodeskInventorPart);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_autodesk_inventor_presentation() {
    let fmt = FileFormat::from_media_type("model/x-ipn");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AutodeskInventorPresentation)), "{:?} does not contain {}", fmt, FileFormat::AutodeskInventorPresentation);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_blender(){
    let fmt = FileFormat::from_media_type("application/x-blender");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Blender)), "{:?} does not contain {}", fmt, FileFormat::Blender);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_cinema4d(){
    let fmt = FileFormat::from_media_type("model/x-c4d");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Cinema4d)), "{:?} does not contain {}", fmt, FileFormat::Cinema4d);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_collaborative_design_activity(){
    let fmt = FileFormat::from_media_type("model/vnd.collada+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CollaborativeDesignActivity)), "{:?} does not contain {}", fmt, FileFormat::CollaborativeDesignActivity);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_design_web_format() {
    let fmt = FileFormat::from_media_type("model/vnd.dwf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DesignWebFormat)), "{:?} does not contain {}", fmt, FileFormat::DesignWebFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_design_web_format_xps() {
    let fmt = FileFormat::from_media_type("model/vnd.dwfx+xps");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DesignWebFormatXps)), "{:?} does not contain {}", fmt, FileFormat::DesignWebFormatXps);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_drawing_exchange_format_ascii() {
    let fmt = FileFormat::from_media_type("application/x-dxf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DrawingExchangeFormatAscii)), "{:?} does not contain {}", fmt, FileFormat::DrawingExchangeFormatAscii);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_drawing_exchange_format_binary() {
    let fmt = FileFormat::from_media_type("application/x-dxf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DrawingExchangeFormatBinary)), "{:?} does not contain {}", fmt, FileFormat::DrawingExchangeFormatBinary);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_extensible3d_graphics(){
    let fmt = FileFormat::from_media_type("model/x3d+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Extensible3d)), "{:?} does not contain {}", fmt, FileFormat::Extensible3d);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_filmbox(){
    let fmt = FileFormat::from_media_type("application/vnd.autodesk.fbx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Filmbox)), "{:?} does not contain {}", fmt, FileFormat::Filmbox);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_fusion360(){
    let fmt = FileFormat::from_media_type("model/x-f3d");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Fusion360)), "{:?} does not contain {}", fmt, FileFormat::Fusion360);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_gl_transmission_format_binary() {
    let fmt = FileFormat::from_media_type("model/gltf-binary");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GlTransmissionFormatBinary)), "{:?} does not contain {}", fmt, FileFormat::GlTransmissionFormatBinary);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_google_draco() {
    let fmt = FileFormat::from_media_type("model/x-draco");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GoogleDraco)), "{:?} does not contain {}", fmt, FileFormat::GoogleDraco);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_initial_graphics_exchange_specification() {
    let fmt = FileFormat::from_media_type("model/iges");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::InitialGraphicsExchangeSpecification)), "{:?} does not contain {}", fmt, FileFormat::InitialGraphicsExchangeSpecification);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_inter_quake_export() {
    let fmt = FileFormat::from_media_type("model/x-iqe");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::InterQuakeExport)), "{:?} does not contain {}", fmt, FileFormat::InterQuakeExport);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_inter_quake_model() {
    let fmt = FileFormat::from_media_type("model/x-iqm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::InterQuakeModel)), "{:?} does not contain {}", fmt, FileFormat::InterQuakeModel);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_magicavoxel(){
    let fmt = FileFormat::from_media_type("model/x-vox");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Magicavoxel)), "{:?} does not contain {}", fmt, FileFormat::Magicavoxel);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_maya_ascii() {
    let fmt = FileFormat::from_media_type("application/x-maya-ascii");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MayaAscii)), "{:?} does not contain {}", fmt, FileFormat::MayaAscii);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_maya_binary() {
    let fmt = FileFormat::from_media_type("application/x-maya-binary");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MayaBinary)), "{:?} does not contain {}", fmt, FileFormat::MayaBinary);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_model3d_ascii() {
    let fmt = FileFormat::from_media_type("text/x-3d-model");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Model3dAscii)), "{:?} does not contain {}", fmt, FileFormat::Model3dAscii);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_model3d_binary() {
    let fmt = FileFormat::from_media_type("model/x-3d-model");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Model3dBinary)), "{:?} does not contain {}", fmt, FileFormat::Model3dBinary);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_opennurbs(){
    let fmt = FileFormat::from_media_type("model/x-3dm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Opennurbs)), "{:?} does not contain {}", fmt, FileFormat::Opennurbs);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_polygon_ascii() {
    let fmt = FileFormat::from_media_type("model/x-ply-ascii");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PolygonAscii)), "{:?} does not contain {}", fmt, FileFormat::PolygonAscii);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_polygon_binary() {
    let fmt = FileFormat::from_media_type("model/x-ply-binary");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PolygonBinary)), "{:?} does not contain {}", fmt, FileFormat::PolygonBinary);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_sketchup(){
    let fmt = FileFormat::from_media_type("application/vnd.sketchup.skp");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Sketchup)), "{:?} does not contain {}", fmt, FileFormat::Sketchup);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_solidworks_assembly() {
    let fmt = FileFormat::from_media_type("model/x-sldasm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SolidworksAssembly)), "{:?} does not contain {}", fmt, FileFormat::SolidworksAssembly);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_solidworks_drawing() {
    let fmt = FileFormat::from_media_type("model/x-slddrw");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SolidworksDrawing)), "{:?} does not contain {}", fmt, FileFormat::SolidworksDrawing);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_solidworks_part() {
    let fmt = FileFormat::from_media_type("model/x-sldprt");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SolidworksPart)), "{:?} does not contain {}", fmt, FileFormat::SolidworksPart);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_spaceclaim_document() {
    let fmt = FileFormat::from_media_type("model/x-scdoc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SpaceclaimDocument)), "{:?} does not contain {}", fmt, FileFormat::SpaceclaimDocument);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_standard_for_the_exchange_of_product_model_data() {
    let fmt = FileFormat::from_media_type("model/step");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::StandardForTheExchangeOfProductModelData)), "{:?} does not contain {}", fmt, FileFormat::StandardForTheExchangeOfProductModelData);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_stereolithography_ascii() {
    let fmt = FileFormat::from_media_type("model/x-stl-ascii");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::StereolithographyAscii)), "{:?} does not contain {}", fmt, FileFormat::StereolithographyAscii);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_three_dimensional_manufacturing_format() {
    let fmt = FileFormat::from_media_type("application/vnd.ms-package.3dmanufacturing-3dmodel+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ThreeDimensionalManufacturingFormat)), "{:?} does not contain {}", fmt, FileFormat::ThreeDimensionalManufacturingFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_three_dimensional_studio() {
    let fmt = FileFormat::from_media_type("application/x-3ds");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ThreeDimensionalStudio)), "{:?} does not contain {}", fmt, FileFormat::ThreeDimensionalStudio);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_three_dimensional_studio_max() {
    let fmt = FileFormat::from_media_type("application/x-max");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ThreeDimensionalStudioMax)), "{:?} does not contain {}", fmt, FileFormat::ThreeDimensionalStudioMax);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_universal3d(){
    let fmt = FileFormat::from_media_type("model/u3d");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Universal3d)), "{:?} does not contain {}", fmt, FileFormat::Universal3d);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_universal_scene_description_ascii() {
    let fmt = FileFormat::from_media_type("model/x-usd");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UniversalSceneDescriptionAscii)), "{:?} does not contain {}", fmt, FileFormat::UniversalSceneDescriptionAscii);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_universal_scene_description_binary() {
    let fmt = FileFormat::from_media_type("model/x-usd");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UniversalSceneDescriptionBinary)), "{:?} does not contain {}", fmt, FileFormat::UniversalSceneDescriptionBinary);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_universal_scene_description_zip() {
    let fmt = FileFormat::from_media_type("model/vnd.usdz+zip");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UniversalSceneDescriptionZip)), "{:?} does not contain {}", fmt, FileFormat::UniversalSceneDescriptionZip);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_virtual_reality_modeling_language() {
    let fmt = FileFormat::from_media_type("model/vrml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::VirtualRealityModelingLanguage)), "{:?} does not contain {}", fmt, FileFormat::VirtualRealityModelingLanguage);
}

