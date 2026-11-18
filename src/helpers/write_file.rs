pub fn write(file_name: String, text: String, exception: String) -> String {

    let filename = file_name.trim();
    let except = exception.trim();

    std::fs::write(filename, text)
        .expect(exception);
}