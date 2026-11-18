pub fn read(exception: String, name: String, extension: String) -> String {
    let except = exception.trim();
    let filename = name.trim();
    let extension = extension.trim();

    let exc = Vec![".txt", ".json", ".bin"];
    if extension.contains(&exc){
        let text = std::fs::read_to_string(filename + extension)
            .expect(except);
        return text;
    }
}