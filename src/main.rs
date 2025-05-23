use std::io::{self, Write};
use std::process::{Command, Child};
use std::env;
use std::path::Path;

/// 命令行解释器结构体
pub struct CliInterpreter {
    current_dir: String,
}

impl CliInterpreter {
    /// 创建新的命令行解释器实例
    pub fn new() -> Self {
        let current_dir = env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("/"))
            .to_string_lossy()
            .to_string();
        
        CliInterpreter { current_dir }
    }

    /// 启动解释器主循环
    pub fn run(&mut self) {
        println!("欢迎使用 Rust CLI 解释器!");
        println!("输入 'help' 查看帮助，输入 'exit' 退出");
        
        loop {
            // 显示提示符
            print!("rust-cli:{}$ ", self.current_dir);
            io::stdout().flush().unwrap();
            
            // 读取用户输入
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let input = input.trim();
                    if input.is_empty() {
                        continue;
                    }
                    
                    // 处理命令
                    match self.process_command(input) {
                        Ok(should_exit) => {
                            if should_exit {
                                println!("再见!");
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("错误: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("读取输入失败: {}", e);
                }
            }
        }
    }

    /// 处理用户输入的命令
    fn process_command(&mut self, input: &str) -> Result<bool, String> {
        let args: Vec<&str> = input.split_whitespace().collect();
        if args.is_empty() {
            return Ok(false);
        }

        let command = args[0];
        let command_args = &args[1..];

        match command {
            "exit" => Ok(true),
            "help" => {
                self.show_help();
                Ok(false)
            }
            "cd" => {
                self.change_directory(command_args)?;
                Ok(false)
            }
            "pwd" => {
                println!("{}", self.current_dir);
                Ok(false)
            }
            "echo" => {
                self.echo_command(command_args);
                Ok(false)
            }
            _ => {
                self.execute_external_command(command, command_args)?;
                Ok(false)
            }
        }
    }

    /// 显示帮助信息
    fn show_help(&self) {
        println!("内置命令:");
        println!("  help    - 显示此帮助信息");
        println!("  exit    - 退出解释器");
        println!("  cd      - 切换目录");
        println!("  pwd     - 显示当前目录");
        println!("  echo    - 输出文本");
        println!("\n外部命令:");
        println!("  ls      - 列出文件");
        println!("  cat     - 显示文件内容");
        println!("  grep    - 搜索文本");
        println!("  等等... (支持所有系统命令)");
    }

    /// 切换目录命令实现
    fn change_directory(&mut self, args: &[&str]) -> Result<(), String> {
        let path = if args.is_empty() {
            // 如果没有参数，切换到home目录
            env::var("HOME").map_err(|_| "无法获取HOME环境变量".to_string())?
        } else {
            args[0].to_string()
        };

        let new_path = if path.starts_with('/') {
            // 绝对路径
            path
        } else {
            // 相对路径
            format!("{}/{}", self.current_dir, path)
        };

        if Path::new(&new_path).exists() && Path::new(&new_path).is_dir() {
            self.current_dir = new_path;
            env::set_current_dir(&self.current_dir)
                .map_err(|e| format!("切换目录失败: {}", e))?;
            Ok(())
        } else {
            Err(format!("目录不存在: {}", new_path))
        }
    }

    /// echo命令实现
    fn echo_command(&self, args: &[&str]) {
        if args.is_empty() {
            println!();
        } else {
            println!("{}", args.join(" "));
        }
    }

    /// 执行外部命令
    fn execute_external_command(&self, command: &str, args: &[&str]) -> Result<(), String> {
        let mut cmd = Command::new(command);
        cmd.args(args);
        cmd.current_dir(&self.current_dir);

        match cmd.spawn() {
            Ok(mut child) => {
                match child.wait() {
                    Ok(status) => {
                        if !status.success() {
                            return Err(format!("命令执行失败，退出码: {:?}", status.code()));
                        }
                    }
                    Err(e) => {
                        return Err(format!("等待子进程失败: {}", e));
                    }
                }
            }
            Err(e) => {
                return Err(format!("命令执行失败: {} ({})", command, e));
            }
        }

        Ok(())
    }
}

/// 主函数
fn main() {
    let mut interpreter = CliInterpreter::new();
    interpreter.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_interpreter_creation() {
        let cli = CliInterpreter::new();
        assert!(!cli.current_dir.is_empty());
    }

    #[test]
    fn test_echo_command() {
        let cli = CliInterpreter::new();
        // 这里可以添加更多的单元测试
        // 由于echo命令只是打印，我们可以测试它不会panic
        cli.echo_command(&["hello", "world"]);
    }
}