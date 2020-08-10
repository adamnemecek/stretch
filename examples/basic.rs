use stretch::prelude::*;

fn main() -> Result<(), Error> {
    let mut stretch = Stretch::new();
    let child = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
        &[],
    )?;

    let child4 = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
        &[],
    )?;

    let child4b = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
        &[],
    )?;

    let child3 = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
        &[child4, child4b],
    )?;

    let child2 = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
        &[child3],
    )?;

    let child1 = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
        &[child2],
    )?;

    let node = stretch.new_node(
        Style {
            size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) },
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        &[child, child1],
    )?;

    stretch.compute_layout(node, Size::undefined())?;
    println!("node: {:#?}", stretch.layout(node)?);
    println!("child: {:#?}", stretch.layout(child)?);
    println!("child: {:#?}", stretch.layout(child1)?);
    println!("child: {:#?}", stretch.layout(child2)?);
    println!("child3: {:#?}", stretch.layout(child3)?);
    println!("child4: {:#?}", stretch.layout(child4)?);
    println!("child4b: {:#?}", stretch.layout(child4b)?);

    println!("{}", stretch.parent(child1).unwrap() == node);
    println!("{}", stretch.parent(child2).unwrap() == child1);

    Ok(())
}
