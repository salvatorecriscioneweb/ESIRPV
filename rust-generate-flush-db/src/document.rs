use barcoders::{generators::image::Image, sym::code128::Code128};
use genpdf::{elements, Size};
use qrcode_generator::QrCodeEcc;
use std::{fs::File, io::Write, path::Path};

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

    fn generate_qr_code(&self, order_id: String) {
        qrcode_generator::to_png_to_file(
            format!("#{}", order_id),
            QrCodeEcc::Low,
            600,
            format!("test_data/qr-{}.png", order_id),
        )
        .unwrap();
    }

    fn generate_barcode(&self, order_id: String, client_data: String) {
        let barcode = Code128::new(format!("À{}{}", order_id, client_data.to_ascii_uppercase()));

        let png = Image::jpeg(200);
        let encoded = barcode.unwrap().encode();

        // Image generators return a Result<Vec<u8>, barcoders::error::Error) of encoded bytes.
        let bytes = png.generate(&encoded[..]).unwrap();

        // Which you can then save to disk.
        let file = File::create(&Path::new(&format!("test_data/{}.jpg", order_id))).unwrap();
        let mut writer = std::io::BufWriter::new(file);
        writer.write(&bytes[..]).expect("");
    }

    pub fn generate(&self, client_data: String, order_id: String) {
        let font_family =
            genpdf::fonts::from_files("./fonts", "Arial", None).expect("Failed to load Arial");
        let mut doc = genpdf::Document::new(font_family);
        // Customize the pages
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_minimal_conformance();
        doc.set_paper_size(Size::new(101, 152));
        doc.set_page_decorator(decorator);
        doc.push(genpdf::elements::Text::new("SHIPPING LABEL"));
        doc.push(genpdf::elements::Break::new(1));

        doc.push(genpdf::elements::Text::new(format!("DEMO DEMO DEMO")));
        doc.push(genpdf::elements::Break::new(1));

        doc.push(genpdf::elements::Paragraph::new(format!(
            "Order#{}",
            order_id
        )));
        doc.push(genpdf::elements::Break::new(1));

        doc.push(genpdf::elements::Paragraph::new("Ship to:"));
        doc.push(genpdf::elements::Paragraph::new(format!("{}", client_data)));

        doc.push(genpdf::elements::Break::new(1));

        self.generate_qr_code(order_id.clone());
        self.generate_barcode(order_id.clone(), client_data.clone());

        let img_qr_pdf =
            elements::Image::from_path(format!("test_data/qr-{}.png", order_id)).unwrap();
        let img_pdf = elements::Image::from_path(format!("test_data/{}.jpg", order_id)).unwrap();

        doc.push(img_pdf);

        doc.push(genpdf::elements::Break::new(2));
        doc.push(img_qr_pdf);

        // Render the document and write it to a file
        doc.render_to_file(format!("test_data/{}.pdf", order_id))
            .expect("Failed to write PDF file");

        std::fs::remove_file(format!("test_data/qr-{}.png", order_id)).expect("Error cleaning");
        std::fs::remove_file(format!("test_data/{}.jpg", order_id)).expect("Error Cleaning")
    }
}
