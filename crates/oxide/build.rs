fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set("FileDescription", "Oxide");
        res.set("ProductName", "Oxide");
        res.set("InternalName", "Oxide");
        res.set_icon("../../assets/icon.ico");
        res.compile().unwrap();
    }
}