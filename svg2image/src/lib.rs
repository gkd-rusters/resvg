// use usvg::SystemFontDB;
use std::path;
use image_rs::{
    ImageBuffer, DynamicImage,
};
pub mod prelude{
    pub use usvg::{
        check_is_svg,
    };
    pub use {
        super::{
            svgsrc2image, svgsdata2image
        },
    };
    // pub use resvg::Image;
}

fn render_svg(
    rtree: &usvg::Tree,
    background_color: Option<usvg::Color>,
    dimension: (Option<f64>, Option<f64>)
) ->Option<(DynamicImage, u32, u32)> {
    let fit_to = match dimension {
        (Some(w), Some(h)) => {
            usvg::FitTo::Transform(w, h)
        },
        (Some(w), None) => {
            usvg::FitTo::Width(w)
        },
        (None, Some(h)) => {
            usvg::FitTo::Height(h)
        },
        _ => {
            usvg::FitTo::Original
        }
    };
    let pixmap_size = fit_to.fit_to(rtree.svg_node().size.to_screen_size())?;
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())?;
    resvg::render(&rtree, fit_to, pixmap.as_mut())?;
    let width = pixmap_size.width();
    let height = pixmap_size.height();
    let dynamic_image = ImageBuffer::from_raw(width, height, pixmap.take()).map(DynamicImage::ImageRgba8)?;

    Some((dynamic_image, width, height))
}

pub fn svgsdata2image(
    data:&[u8],
    background_color: Option<usvg::Color>,
    dimension: (Option<f64>, Option<f64>))
    -> Option<(DynamicImage, u32, u32)>
{
    use usvg::Tree;
    let opt = usvg::Options::default();

    let rtree = Tree::from_data(data, &opt).ok()?;
    render_svg(&rtree, background_color, dimension)
}


pub fn svgsrc2image<P: AsRef<path::Path>>(
    path:P,
    background_color: Option<usvg::Color>,
    dimension: (Option<f64>, Option<f64>))
    -> Option<(DynamicImage, u32, u32)>
{
    use usvg::Tree;

    let mut opt = usvg::Options::default();
    opt.resources_dir = Some(path.as_ref().into());

    let rtree = Tree::from_file(path, &opt).ok()?;
    render_svg(&rtree, background_color, dimension)

}
mod tests{
    // use usvg::SystemFontDB;
    use super::*;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;


    #[test]
    fn test_check_svgfile(){
        let path = "/Users/admin/repo/gui/resvg/.github/chart.svg";
        let res = prelude::check_is_svg(path);
        assert!(res);
    }

    #[test]
    fn test_dimesions_src(){

        // let path = "/Users/admin/repo/gui/resvg/warning.svg";
        let path = "/Users/admin/repo/gui/resvg/warning.svg";
        // let mut opt = usvg::Options::default();
        // opt.path = Some(args[1].clone().into());
        let dimension = (Some(100f64), Some(100f64));
        // opt.dimension =  dimension;
        let res = svgsrc2image(path, None, dimension).unwrap();
        let out_img = res.0;
        use image_rs::ImageFormat;
        out_img.save_with_format("../output.png", ImageFormat::Png).expect("failed to store png");
        println!("{:?}", res.1);
    }
    #[test]
    fn test_dimesions_data(){

        // let path = "/Users/admin/repo/gui/resvg/warning.svg";
        let path = "/Users/admin/repo/gui/resvg/jpeg_base64.svg";
        let mut f = File::open(path).expect("unable to open file");
        let mut buffer :Vec<u8>= Vec::new();

        // read the whole file
        f.read_to_end(&mut buffer).expect("faliled to read to buffer");
        let dimension = (Some(100f64), Some(100f64));
        let res = svgsdata2image(buffer.as_ref(), None, dimension).unwrap();
        let out_img = res.0;
        use image_rs::ImageFormat;
        out_img.save_with_format("../output.png", ImageFormat::Png).expect("failed to store png");
        println!("{:?}", res.1);
    }
}

