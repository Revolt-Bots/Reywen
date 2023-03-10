    // a sandbox for experimenting with OOP design patterns for abstraction and DX
    // lets create a simple hello world thingo

//use crate::{lib::{conf::Auth, message::{RMessage, Masquerade, RMessagePayload}}, rev_x::{rev_send, rev_del_2}};
//use crate::send;



use crate::{lib::{conf::Auth, message::{RMessage, Masquerade, RMessagePayload}}, rev_x::rev_send};


/*
#[derive(Debug)]
pub enum ReyCLI{

    Send{content: & 'static str},
    SendMasq{content:  & 'static str, masq: Masquerade},
    Delete{message: String},
    None,
    
}
pub async fn oop_main(auth: Auth, input_message: RMessage ) {
    
impl ReyCLI {

    
    pub async fn run(&self, auth: Auth, message_input: RMessage) {
        match self {

            Self::Send{content } => {
                send(&auth.token, &message_input, content ).await
            },
                        
            Self::SendMasq { content, masq } => {
      
                let payload = RMessagePayload {
                    content: Some(content.to_string()),
                    attachments: None,
                    replies: None,
                    masquerade: Some(masq.to_owned()),   
                    
                };
                rev_send(&auth.token, &message_input.channel, payload).await          
            },
            
            Self::Delete{message} => {
                
                
                rev_del_2(&auth.token, &message_input.channel, message).await;
            },

            
            Self::None => {},
            
        };
    }


}

    let config = Oop{enabled: false};
    
    if config.enabled == false {
        return
    };

      let convec: Vec<String> = input_message.content.clone().unwrap().split_whitespace().map(str::to_string).collect();
      

    let masq: Masquerade  = Masquerade {
        avatar: Some("https://autumn.revolt.chat/avatars/tpDMc0zLiHzg9ZBLCHwF-7a50PRWwy9dsUMZaGi_2m".to_string()),
        name: Some("Reywen-MASQ".to_string()),
        colour: None,            
    };
         
  
         
    match &convec[0] as &str {
        
        // send message normally
        "?mog" => ReyCLI::Send { content: (":01G7MT5B978E360NB6VWAS9SJ6:") },
        "?ver" => ReyCLI::Send { content: ("Rolling release ([here](https://github.com/toastxc/Reywen-Revolt))") },
        // send message with masquerade (requires a masq object)
        "?mogus" => ReyCLI::SendMasq { content: ("sus"), masq: (masq) },
        // deletes a given message
        "?del" => ReyCLI::Delete { message: (convec[1].clone()) },
        //nothing
        _ => ReyCLI::None,
    }.run(auth, input_message).await;
    
    
    
    
    
    
  
    
}

*/


#[derive(Debug, Clone, Default)]
pub struct Reywen {
  auth: Auth
}

impl Reywen {
    
    pub fn new(auth: Auth) -> Self {
        Reywen
        {
            auth
        }
    }
    
  
    pub async fn send(self, payload: RMessagePayload, channel: &str) -> Self {
        //self.send = payload;
        
        rev_send(&self.auth.token, channel, payload).await;
        self
    }
}

impl RMessagePayload {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, content: &'static str) -> Self {
        self.content = Some(content.to_string());
        self
    }
    
    pub fn masq(mut self, masq: Masquerade) -> Self {
        self.masquerade = Some(masq);
        self
        
    }
}




pub async fn oop_main(auth: Auth, input_message: RMessage) {
    
    
    let client = Reywen::new(auth);
  
    let payload  = RMessagePayload::new()
        .content("yayaya");
   
    if input_message.content == Some("?tester".to_string()) {
        client.send(payload, &input_message.channel).await;
    }
    
}
    
  
