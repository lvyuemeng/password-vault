#[macro_export]
macro_rules! handle_control_flow {
    ($line:expr, $($respond:expr),+) => {
        {
            let mut control_flow = $line;
            $(
                match control_flow {
                    ControlFlow::Continue => continue,
                    ControlFlow::Quit => break,
                    ControlFlow::Next(line) => {
                        control_flow = $respond(line)?;
                    }
                }
            )+
        }
    };
}

#[macro_export]
macro_rules! call_control_flow {
    ($sv:expr) => {
        match $sv {
            Control::Continue => return Ok(Control::Continue),
            Control::Quit => return Ok(Control::Quit),
            Control::Next(res) => res,
        }
    };
}
