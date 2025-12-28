macro_rules! deps {
    () => {
        Step!();
        CommandStatus!();
    };
}

macro_rules! overwrite_trycmd_status {
    () => {
        deps!();
        fn overwrite_trycmd_status (exit : Option < std :: process :: ExitStatus > , step : & Step , stdout_line_nums : & mut std :: ops :: Range < usize > , normalized : & mut String ,) -> Result < () , crate :: Error > { let status = match exit { Some (status) => status , _ => { return Ok (()) ; } } ; let formatted_status = if let Some (code) = status . code () { if status . success () { if let (true , Some (line_num)) = (step . expected_status != Some (CommandStatus :: Success) , step . expected_status_source ,) { replace_lines (normalized , line_num .. (line_num + 1) , "") ? ; * stdout_line_nums = (stdout_line_nums . start - 1) .. (stdout_line_nums . end - 1) ; } None } else { match step . expected_status { Some (CommandStatus :: Success | CommandStatus :: Interrupted) => { Some (format ! ("? {code}")) } Some (CommandStatus :: Code (expected)) if expected != code => { Some (format ! ("? {code}")) } _ => None , } } } else { if step . expected_status == Some (CommandStatus :: Interrupted) { None } else { Some ("? interrupted" . into ()) } } ; if let Some (status) = formatted_status { if let Some (line_num) = step . expected_status_source { replace_lines (normalized , line_num .. (line_num + 1) , & status) ? ; } else { let line_num = stdout_line_nums . start ; replace_lines (normalized , line_num .. line_num , & status) ? ; * stdout_line_nums = (line_num + 1) .. (stdout_line_nums . end + 1) ; } } Ok (()) }
    };
}

overwrite_trycmd_status!()