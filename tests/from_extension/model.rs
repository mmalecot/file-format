use file_format::FileFormat;

#[test]
#[cfg(feature = "from-extension")]
fn test_additive_manufacturing_format(){
    let fmt = FileFormat::from_extension("amf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AdditiveManufacturingFormat)), "{:?} does not contain {}", fmt, FileFormat::AdditiveManufacturingFormat);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_autocad_drawing() {
    let fmt = FileFormat::from_extension("dwg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AutocadDrawing)), "{:?} does not contain {}", fmt, FileFormat::AutocadDrawing);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_autodesk123d(){
    let fmt = FileFormat::from_extension("123dx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Autodesk123d)), "{:?} does not contain {}", fmt, FileFormat::Autodesk123d);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_autodesk_alias() {
    let fmt = FileFormat::from_extension("wire");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AutodeskAlias)), "{:?} does not contain {}", fmt, FileFormat::AutodeskAlias);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_autodesk_inventor_assembly() {
    let fmt = FileFormat::from_extension("iam");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AutodeskInventorAssembly)), "{:?} does not contain {}", fmt, FileFormat::AutodeskInventorAssembly);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_autodesk_inventor_drawing() {
    let fmt = FileFormat::from_extension("idw");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AutodeskInventorDrawing)), "{:?} does not contain {}", fmt, FileFormat::AutodeskInventorDrawing);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_autodesk_inventor_part() {
    let fmt = FileFormat::from_extension("ipt");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AutodeskInventorPart)), "{:?} does not contain {}", fmt, FileFormat::AutodeskInventorPart);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_autodesk_inventor_presentation() {
    let fmt = FileFormat::from_extension("ipn");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AutodeskInventorPresentation)), "{:?} does not contain {}", fmt, FileFormat::AutodeskInventorPresentation);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_blender(){
    let fmt = FileFormat::from_extension("blend");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Blender)), "{:?} does not contain {}", fmt, FileFormat::Blender);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_cinema4d(){
    let fmt = FileFormat::from_extension("c4d");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Cinema4d)), "{:?} does not contain {}", fmt, FileFormat::Cinema4d);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_collaborative_design_activity(){
    let fmt = FileFormat::from_extension("dae");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CollaborativeDesignActivity)), "{:?} does not contain {}", fmt, FileFormat::CollaborativeDesignActivity);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_design_web_format() {
    let fmt = FileFormat::from_extension("dwf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DesignWebFormat)), "{:?} does not contain {}", fmt, FileFormat::DesignWebFormat);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_design_web_format_xps() {
    let fmt = FileFormat::from_extension("dwfx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DesignWebFormatXps)), "{:?} does not contain {}", fmt, FileFormat::DesignWebFormatXps);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_drawing_exchange_format_ascii() {
    let fmt = FileFormat::from_extension("dxf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DrawingExchangeFormatAscii)), "{:?} does not contain {}", fmt, FileFormat::DrawingExchangeFormatAscii);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_drawing_exchange_format_binary() {
    let fmt = FileFormat::from_extension("dxf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DrawingExchangeFormatBinary)), "{:?} does not contain {}", fmt, FileFormat::DrawingExchangeFormatBinary);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_extensible3d_graphics(){
    let fmt = FileFormat::from_extension("x3d");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Extensible3d)), "{:?} does not contain {}", fmt, FileFormat::Extensible3d);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_filmbox(){
    let fmt = FileFormat::from_extension("fbx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Filmbox)), "{:?} does not contain {}", fmt, FileFormat::Filmbox);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_fusion360(){
    let fmt = FileFormat::from_extension("f3d");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Fusion360)), "{:?} does not contain {}", fmt, FileFormat::Fusion360);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_gl_transmission_format_binary() {
    let fmt = FileFormat::from_extension("glb");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GlTransmissionFormatBinary)), "{:?} does not contain {}", fmt, FileFormat::GlTransmissionFormatBinary);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_google_draco() {
    let fmt = FileFormat::from_extension("drc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GoogleDraco)), "{:?} does not contain {}", fmt, FileFormat::GoogleDraco);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_initial_graphics_exchange_specification() {
    let fmt = FileFormat::from_extension("iges");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::InitialGraphicsExchangeSpecification)), "{:?} does not contain {}", fmt, FileFormat::InitialGraphicsExchangeSpecification);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_inter_quake_export() {
    let fmt = FileFormat::from_extension("iqe");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::InterQuakeExport)), "{:?} does not contain {}", fmt, FileFormat::InterQuakeExport);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_inter_quake_model() {
    let fmt = FileFormat::from_extension("iqm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::InterQuakeModel)), "{:?} does not contain {}", fmt, FileFormat::InterQuakeModel);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_magicavoxel(){
    let fmt = FileFormat::from_extension("vox");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Magicavoxel)), "{:?} does not contain {}", fmt, FileFormat::Magicavoxel);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_maya_ascii() {
    let fmt = FileFormat::from_extension("ma");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MayaAscii)), "{:?} does not contain {}", fmt, FileFormat::MayaAscii);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_maya_binary() {
    let fmt = FileFormat::from_extension("mb");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MayaBinary)), "{:?} does not contain {}", fmt, FileFormat::MayaBinary);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_model3d_ascii() {
    let fmt = FileFormat::from_extension("a3d");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Model3dAscii)), "{:?} does not contain {}", fmt, FileFormat::Model3dAscii);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_model3d_binary() {
    let fmt = FileFormat::from_extension("m3d");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Model3dBinary)), "{:?} does not contain {}", fmt, FileFormat::Model3dBinary);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_opennurbs(){
    let fmt = FileFormat::from_extension("3dm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Opennurbs)), "{:?} does not contain {}", fmt, FileFormat::Opennurbs);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_polygon_ascii() {
    let fmt = FileFormat::from_extension("ply");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PolygonAscii)), "{:?} does not contain {}", fmt, FileFormat::PolygonAscii);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_polygon_binary() {
    let fmt = FileFormat::from_extension("ply");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PolygonBinary)), "{:?} does not contain {}", fmt, FileFormat::PolygonBinary);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_sketchup(){
    let fmt = FileFormat::from_extension("skp");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Sketchup)), "{:?} does not contain {}", fmt, FileFormat::Sketchup);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_solidworks_assembly() {
    let fmt = FileFormat::from_extension("sldasm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SolidworksAssembly)), "{:?} does not contain {}", fmt, FileFormat::SolidworksAssembly);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_solidworks_drawing() {
    let fmt = FileFormat::from_extension("slddrw");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SolidworksDrawing)), "{:?} does not contain {}", fmt, FileFormat::SolidworksDrawing);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_solidworks_part() {
    let fmt = FileFormat::from_extension("sldprt");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SolidworksPart)), "{:?} does not contain {}", fmt, FileFormat::SolidworksPart);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_spaceclaim_document() {
    let fmt = FileFormat::from_extension("scdoc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SpaceclaimDocument)), "{:?} does not contain {}", fmt, FileFormat::SpaceclaimDocument);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_standard_for_the_exchange_of_product_model_data() {
    let fmt = FileFormat::from_extension("step");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::StandardForTheExchangeOfProductModelData)), "{:?} does not contain {}", fmt, FileFormat::StandardForTheExchangeOfProductModelData);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_stereolithography_ascii() {
    let fmt = FileFormat::from_extension("stl");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::StereolithographyAscii)), "{:?} does not contain {}", fmt, FileFormat::StereolithographyAscii);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_three_dimensional_manufacturing_format() {
    let fmt = FileFormat::from_extension("3mf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ThreeDimensionalManufacturingFormat)), "{:?} does not contain {}", fmt, FileFormat::ThreeDimensionalManufacturingFormat);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_three_dimensional_studio() {
    let fmt = FileFormat::from_extension("3ds");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ThreeDimensionalStudio)), "{:?} does not contain {}", fmt, FileFormat::ThreeDimensionalStudio);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_three_dimensional_studio_max() {
    let fmt = FileFormat::from_extension("max");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ThreeDimensionalStudioMax)), "{:?} does not contain {}", fmt, FileFormat::ThreeDimensionalStudioMax);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_universal3d(){
    let fmt = FileFormat::from_extension("u3d");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Universal3d)), "{:?} does not contain {}", fmt, FileFormat::Universal3d);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_universal_scene_description_ascii() {
    let fmt = FileFormat::from_extension("usda");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UniversalSceneDescriptionAscii)), "{:?} does not contain {}", fmt, FileFormat::UniversalSceneDescriptionAscii);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_universal_scene_description_binary() {
    let fmt = FileFormat::from_extension("usdc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UniversalSceneDescriptionBinary)), "{:?} does not contain {}", fmt, FileFormat::UniversalSceneDescriptionBinary);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_universal_scene_description_zip() {
    let fmt = FileFormat::from_extension("usdz");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UniversalSceneDescriptionZip)), "{:?} does not contain {}", fmt, FileFormat::UniversalSceneDescriptionZip);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_virtual_reality_modeling_language() {
    let fmt = FileFormat::from_extension("wrl");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::VirtualRealityModelingLanguage)), "{:?} does not contain {}", fmt, FileFormat::VirtualRealityModelingLanguage);
}

