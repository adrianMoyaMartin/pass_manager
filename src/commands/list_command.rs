use std::vec;

pub fn list_command(buffer: Vec<u8>) {
    let mut last_blank: i32 = 10;
    let mut last_index: usize = 0;
    let buffer_len = buffer.len();

    let mut name_list: Vec<Vec<u8>> = vec![];

    for (i, x) in buffer.into_iter().enumerate() {
        if x == 0 {
            last_blank = 0;

            let data_to_print = &name_list[last_index - 1];
            let text_to_print = String::from_utf8(data_to_print.to_vec()).expect(
                "failed whilst converting unicode"
            );

            println!("{}", text_to_print);
        } else if x == 10 || last_index == 0 {
            if i != buffer_len - 1 {
                last_blank = 10;
                name_list.push(vec![]);
            }

            last_index = name_list.len();
        }

        if last_blank == 10 && x != 10 {
            name_list[last_index - 1].push(x);
        }
    }
}
