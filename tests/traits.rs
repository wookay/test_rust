struct Agent {
    name: String
}

trait Ask {
    fn what_is_your_name(&self) -> &String;
}

impl Ask for Agent {
    fn what_is_your_name(&self) -> &String {
        &(self.name)
    }
}

#[test]
fn rust_traits() {

    let agent = Agent { name: "Anby".to_string() };

    assert_eq!(agent.name, "Anby");
    assert_eq!(&(agent.name), &"Anby");
    assert_eq!(&(agent).name, &"Anby");
    assert_eq!(agent.name, "Anby".to_string());

    assert_eq!(agent.what_is_your_name(), "Anby");
    assert_eq!(agent.what_is_your_name(), &"Anby");
}
