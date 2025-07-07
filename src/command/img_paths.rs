use rand::seq::IndexedRandom;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use tracing::instrument;

/// iterates through vec of paths and get a random img from each dir
pub(super) fn random_img<P: AsRef<Path>>(paths: Vec<P>) -> PathBuf {
    let rando_paths: Vec<PathBuf> = paths
        .iter()
        .map(|path| random_img_single_path(path))
        .collect();

    let mut rng = rand::rng();
    let rand_index: Vec<usize> = (0..rando_paths.len()).collect();
    let rand_index = rand_index.choose(&mut rng).unwrap();
    let img = rando_paths.get(*rand_index);
    img.unwrap().to_path_buf()
}

/// NOTE: this only scans for direct files in the directory and won't walk
/// down the tree
#[instrument(skip(path), ret)]
fn random_img_single_path<P: AsRef<Path>>(path: P) -> PathBuf {
    // if path is a readable file, ret
    if path.as_ref().is_file() {
        return path.as_ref().into();
    }
    // if path is valid dir, do rand
    if path.as_ref().is_dir() {
        let entries = read_dir(path.as_ref()).unwrap(); // TODO: unwrap
        // NOTE: any abitrary file works here, need to implement media filter
        let mut rng = rand::rng();

        let imgs: Vec<PathBuf> = entries.filter_map(|s| s.ok()).map(|e| e.path()).collect();

        let rand_index: Vec<usize> = (0..imgs.len()).collect();
        let rand_index = rand_index.choose(&mut rng).unwrap();
        let img = imgs.get(*rand_index);

        return img.unwrap().to_path_buf();
    }

    path.as_ref().into()
}
