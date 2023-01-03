use jenkins_api::JenkinsBuilder;
use macroquad::prelude::*;
use std::sync::mpsc;

#[macroquad::main("Train-Helper")]
async fn main() {
    let jenkins = JenkinsBuilder::new("https://crackerjack.intel.com/")
        .with_user("jczaja", Some("_Bandzior65"))
        .build().unwrap();

    let (sender, reciever) = mpsc::channel::<jenkins_api::nodes::ComputerSet>();

    std::thread::spawn(move || loop {
        let cs = jenkins.get_nodes().unwrap();
        sender
            .send(cs)
            .expect("Error sending data");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    });

    let mut cs = reciever.recv().expect("Sender hanged up");
    
    const FONT_SIZE: f32 = 30.0;
    loop {
        let mut text_position: f32 = FONT_SIZE;

         cs = match reciever.try_recv() {
            Ok(cs) => cs,
            Err(std::sync::mpsc::TryRecvError::Empty) => cs,
            Err(_) => panic!("Communication error"),
        };

        clear_background(WHITE);

        cs.computers.iter().for_each(|x| { 
            if x.idle == true && x.offline == false { 
                let info = format!("{}, IDLE",x.display_name);
                draw_text(&info, 20.0, text_position, FONT_SIZE, ORANGE);
                text_position += FONT_SIZE;
            }
        });    

        cs.computers.iter().for_each(|x| { 
            if x.idle == false && x.offline == false { 
                let info = format!("{}, BUSY",x.display_name);
                draw_text(&info, 20.0, text_position, FONT_SIZE, BLUE);
                text_position += FONT_SIZE;
            }
        });    

        cs.computers.iter().for_each(|x| { 
            if x.offline == true { 
                let info = format!("{}, OFFLINE",x.display_name);
                draw_text(&info, 20.0, text_position, FONT_SIZE, GRAY);
                text_position += FONT_SIZE;
            }
        });    
        next_frame().await
    }

}
