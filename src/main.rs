/* Автор: a2p1k02
 * Версия: 0.5.0
 * Сделано на Rust с любовью <3
 * Name: r2shell
 * 
 * Мне было лень писать тут на английском, т.к. я заколебался переписывать этот шелл уже в сотый раз
 * и всё на разных языках тут был и Си и С++, но оригинал был на Python, 
 * а т.к. я люблю ранее перечисленные языки я решил пописать на Rust и это было шикарно, и
 * вроде даже нервов убито меньше, чем обычно.
 * 
 * Если кто-то захочет тут исправить что-то, добавьте к счётчику ниже единицу, 
 * так я буду знать кому интересно было "ковырять" мой проект
 * 
 * Изначально, было 1!
 * 
 * СчётчикСломанныхНервов = 13;
 * 
 */

use std::process::{Command, exit};
use std::io::{stdin, stdout, Write};
use std::env;


fn help(help: &str){
    if help.trim() == "help" {
        println!("Это шелл созданный a2p1k02, изначально написанный на Python");
        println!("Теперь это тот же шелл с теми же функциями, но уже не Rust");
    }  
}


fn cd(command: &str) {
    let home = String::from("/home/");
    let name = whoami::username();
    let path = home.to_string() + &name.to_string();

    if command == "cd" || command == "cd " {
        env::set_current_dir(path).unwrap();
    } else {
        env::set_current_dir(&command[3..].to_string()).unwrap();
    }
}


fn main() {

    let name = whoami::username(); //username
    let os = whoami::hostname(); // hostname

    loop {    
        print!("{}@{} $ ", name, os);
        stdout().flush().unwrap();    
        let mut input = String::new(); //Создаём переменную-ввод, для работы с командами
        stdin().read_line(&mut input).unwrap(); //Считываем её с клавиатуры.
        
        let mut something = input.trim().split_whitespace(); //Эта команда и будет выполнять наши команды, я надеюсь...
        let command = something.next().unwrap();
        let args = something;

        if command == "exit" {
            exit(0); // Тут типо варнинг, зато я знаю, что программа завершилась верно.
        } else if command == "help" {
            help(command); //Обычный вызов help'а
        // } else if command == format!("cd {command}", command = &command[3..].to_string()) {
        //     cd(command); //Тут было сломано 100500 нервов
        } else {
            let mut child = Command::new(command).args(args).spawn().unwrap(); //Выполнение команд
            child.wait().unwrap(); //Ожидание следующей команды, новой не будет, пока есть старая
        }
    }
}
