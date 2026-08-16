fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icones/docdg.ico");
        res.compile().expect("ressource Windows");
    }
}
