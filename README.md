## Клонирование проекта
```sh
git clone --recurse-submodules https://github.com/Rev1le/bftp_app
cd  bftp_app
```
## Установите и запустите DEV режим клиента
```sh
npm i
npm run tauri dev
```
## Сборка клиента
```sh
npm i
npm run tauri build
```
Проект распологается по пути .src-tauri/target/release/bftp_app.exe
