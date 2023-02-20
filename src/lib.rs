pub const SERVER_PORT: u16 = 4812;

pub const OP_SHUTDOWN: u8 = 1;
pub const OP_OK: u8 = 0;
pub const OP_NOT_OK: u8 = 255;


pub fn parse_mac_from_str(address_str: String) -> Result<[u8;6], std::num::ParseIntError> {
    let mut addr = [0;6];

    for (hex_str, num) in address_str.split('-').zip(addr.iter_mut()) {
        let hex = u8::from_str_radix(hex_str, 16)?;
        *num = hex;
    }

    Ok(addr)
}


pub const HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>RemoteTools - By Clemens</title>
</head>
<body>
    <script>
        function sendShutdown() {
            const http = new XMLHttpRequest();
            http.open("POST", "shutdown", true)

            http.onreadystatechange = function () {
                if (this.readyState === 4 && this.status !== 200) {
                    alert(this.status)
                }
            }

            http.send();
        }

        function sendStartUp() {
            const http = new XMLHttpRequest();
            http.open("POST", "startup", true)

            http.onreadystatechange = function () {
                if (this.readyState === 4 && this.status !== 200) {
                    alert(this.readyState)
                }
            }

            http.send();
        }
    </script>

    <button onclick="sendShutdown()">Shutdown</button>

    <button onclick="sendStartUp()">Start Up</button>
</body>
</html>"#;