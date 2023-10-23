use opencv::{highgui, imgcodecs, prelude::*, ml::*, core::*};

pub fn affiche_image(image: &str) -> opencv::Result<()> {
	let image = imgcodecs::imread(&image, imgcodecs::IMREAD_GRAYSCALE)?;
	if image.empty() {
		panic!("L'image n'a pas pue être ouverte.");
	}

	highgui::imshow("image :", &image)?;
	highgui::wait_key_def()?;

	Ok(())
}


fn main() {
	let image: &str = "cat.png";
	let _result = affiche_image(image);

	let image = imgcodecs::imread(&image, imgcodecs::IMREAD_GRAYSCALE);

	println!("{:?}", image);

	//let mut knearest_network = KNearest::create();
	//assert!(StatModelTraitConst::empty(&knn));
	//knearest_network.train()?;
}
