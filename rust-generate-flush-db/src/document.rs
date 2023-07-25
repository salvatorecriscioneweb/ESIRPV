use genpdf::{elements, fonts};
use image::RgbImage;

#[derive(Debug)]
pub struct Document {
    pub id: String,
    pub file: String,
    pub hash: String,
    pub timestamp: String,
}

impl Document {
    pub fn default() -> Document {
        Document {
            id: String::from(""),
            file: String::from(""),
            hash: String::from(""),
            timestamp: String::from(""),
        }
    }
    pub fn clone(self) -> Self {
        Self {
            id: self.id,
            file: self.file,
            hash: self.hash,
            timestamp: self.timestamp,
        }
    }
    pub fn generate(&self, client_data: String, order_id: String) {
        let font_family =
            genpdf::fonts::from_files("./fonts", "Arial", None).expect("Failed to load Arial");
        let mut doc = genpdf::Document::new(font_family);
        // Customize the pages
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_minimal_conformance();
        doc.set_page_decorator(decorator);
        doc.push(genpdf::elements::Paragraph::new(format!(
            "This is a demo document. {} with {}",
            order_id, client_data
        )));

        let mut image = RgbImage::new(100, 100);
        let red = image::Rgb([255, 0, 0]);

        for (_, _, pixel) in image.enumerate_pixels_mut() {
            *pixel = red;
        }

        image.save("test.png");

        let img_pdf = elements::Image::from_path("test.png").unwrap();
        doc.push(img_pdf);
        // Render the document and write it to a file
        doc.render_to_file("output.pdf")
            .expect("Failed to write PDF file");
    }
}
