use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Apple iPhone 17 Pro Max 256GB - Cosmic Orange - Unlocked".to_string(),
            price: 9999.99,
            description: "Apple iPhone 17 Pro Max. The most powerful iPhone ever. Brilliant 6.9-inch display, aluminum unibody design, A19 Pro chip, all 48MP rear cameras and best-ever battery life.".to_string(),
            image: "/Apple iPhone 17 Pro Max.png".to_string()
        },
        Product {
            id: 2,
            name: "Dreame X40 Ultra".to_string(),
            price: 699.99,
            description: "Handle your cleaning tasks smartly with the Dreame X40 Ultra robot vacuum and mop. With 12,000Pa of powerful suction, it picks up pet hair and dirt in few passes. SideReach brush effortlessly cleans corners and furniture legs, while MopExtend RoboSwing technology ensures deep, thorough cleaning. Carpets stay dry thanks to its liftable mop and customizable zones.".to_string(),
            image: "/Dreame X40 Ultra.png".to_string()
        },
        Product {
            id: 3,
            name: "LG 4K HDR OLED Smart TV".to_string(),
            price: 1399.99,
            description: "Enjoy cinematic visuals at home with the LG 4K UHD smart TV. This OLED evo C5 series TV runs on the powerful Alpha 9 AI processor Gen8 for smart performance and superior picture quality. Revel in lifelike 4K resolution, vibrant Dolby Vision, and perfect blacks for true immersion. Ultra-fast 120Hz refresh rate provides fluid motion and reduces blur.".to_string(),
            image: "/LG 4K HDR OLED Smart TV.png".to_string()
        }
    ]
}