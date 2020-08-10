use stretch::prelude::*;

fn main() -> Result<(), Error> {
    let mut stretch = Stretch::new();
    let child = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
        &[],
    )?;

    let child2 = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
        &[],
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

    Ok(())
}
