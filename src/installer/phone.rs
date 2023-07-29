use std::{marker::PhantomData, process::Command, fmt::Display};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Phone<State = Disconnected> {
    pub name: String,
    id: String,
    board: String,
    state: PhantomData<State>,
}

pub struct Disconnected;
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Connected;

// NOTE
// The view function is called twice
// DO NOT PUT CODE HERE WHICH SHOULD NOT RUN TWICE
// example includes code to reboot the system
impl Phone<Disconnected> {
    pub fn get_devices() -> Vec<Phone<Connected>> {
        let output = Command::new("adb").arg("devices").output().unwrap();

        let devices = String::from_utf8(output.stdout).unwrap();

        let result: Vec<Phone<Connected>> = devices
            .split("\n")
            .filter(|item| item.len() > 1)
            .skip(1)
            .filter_map(|item| {
                let mut item_iter = item.split("\t");
                let id = item_iter.next().unwrap();
                let status = item_iter.next().unwrap();

                if status == "device" {
                    let mut phone = Phone::new(id);

                    phone.set_name();

                    return Some(phone);
                }

                return None;
            })
            .collect();

        return result;
    }
}

impl Phone<Connected> {
    pub fn new(id: &str) -> Self {
        return Self {
            id: id.to_string(),
            name: "Unknown".to_string(),
            ..Default::default()
        };
    }

    pub fn set_name(&mut self) {
        self.board = self.get_prop("ro.product.vendor.device");
        self.name = self.get_prop("persist.sys.nt.device.name");
        println!("{}", self.name);
    }


    pub fn install() -> Result<(), ()> {

        return Ok(());
    }

    fn get_prop(&self, prop: &str) -> String {
        let output = Command::new("adb")
            .args(["-s", &self.id, "shell", "getprop", prop])
            .output()
            .unwrap();

        return String::from_utf8(output.stdout).unwrap();
    }

}

impl Display for Phone<Connected> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
