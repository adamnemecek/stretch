use stretch::prelude::*;

fn main() -> Result<(), Error> {
    println!("{:?}", std::mem::size_of::<stretch::prelude::Layout>());
    let mut stretch = Stretch::new();
    let toolbar_view = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(1.0), height: Dimension::Percent(0.25) }, ..Default::default() },
        &[],
    )?;

    let list_view = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(0.25), height: Dimension::Auto }, ..Default::default() },
        &[],
    )?;

    let grid_view = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
        &[],
    )?;

    let inspector_view = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(0.25), height: Dimension::Auto }, ..Default::default() },
        &[],
    )?;

    let container_view = stretch.new_node(
        Style {
            size: Size { width: Dimension::Percent(1.0), height: Dimension::Percent(0.75) },
            flex_direction: stretch::prelude::FlexDirection::Row,
            ..Default::default()
        },
        &[list_view, grid_view, inspector_view],
        // &[]
    )?;

    let root = stretch.new_node(
        Style {
            size: Size { width: Dimension::Points(800.0), height: Dimension::Points(600.0) },
            flex_direction: stretch::prelude::FlexDirection::Column,
            ..Default::default()
        },
        &[toolbar_view, container_view],
    )?;

    stretch.compute_layout(root, Size::undefined())?;
    println!("node: {:#?}", stretch.layout(root)?);
    println!("toolbar_view: {:?}", stretch.layout(toolbar_view)?);
    println!("list_view: {:?}", stretch.layout(list_view)?);
    println!("grid_view: {:?}", stretch.layout(grid_view)?);
    println!("inspector_view: {:?}", stretch.layout(inspector_view)?);
    println!("container_view: {:?}", stretch.layout(container_view)?);

    println!("parent: {:?}", stretch.parent(toolbar_view));

    // println!("child: {:#?}", stretch.layout(child)?);
    // println!("child: {:#?}", stretch.layout(child1)?);
    // println!("child: {:#?}", stretch.layout(child2)?);
    // println!("child3: {:#?}", stretch.layout(child3)?);
    // println!("child4: {:#?}", stretch.layout(child4)?);
    // println!("child4b: {:#?}", stretch.layout(child4b)?);

    // println!("{}", stretch.parent(child1).unwrap() == node);
    // println!("{}", stretch.parent(child2).unwrap() == child1);

    Ok(())
}
