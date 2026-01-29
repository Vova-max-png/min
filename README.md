# Min - bridge between max and telegram

## Use max without using it

### Here you will find installation guide and information for contributors

### Warning: this is pre-alpha test, you have to understand that the program is not stable and can break down at any time

## Installation:

* ## Install and configure min
    1. ### Click releases button and download the latest available version
    2. ### After it extract the downloaded .zip file somewhere(it's preferred to do so in any folder, not on your desktop)
    3. ### [Install special extension](https://github.com/Vova-max-png/maxogrambext) to retrieve your max's token, just download the source code and import it in your browser. Example for google chrome and some other browser: go to chrome://extensions -> enable developer mode in the right top corner -> press "load unpacked extension" button in the left top corner. Choose the folder you've just downloaded.
    4. ### Open [max](https://wem.max.ru) and log in there using your phone(yes, it's required just once, then you can delete the app) and open extension(extension should be opened with max page opened too). Wait until status is active and press start button. After pressing it press copy to clipboard button. 
    5. ### Return to min you've just downloaded from this page and paste this to you .env file:
    ```makefile
    TOKEN="<Your max token, that you've just copied>"
    T_TOKEN="<Your telegram token, we will paste it later>"
    ```
    6. ### After doing it create telegram bot using [Botfather](https://telegram.me/BotFather), choose the appropriate name for it, if you want you can choose an icon for it. Then get it's token.
    7. ### Paste in in your .env file 
    ```makefile
    TOKEN="Some token you have already placed here"
    T_TOKEN="<Your telegram bot's token>"
    ```
    8. ### Run the .exe file you've downloaded from this page. If there are any troubles try changing appVersion field in config.json and increase the last number in the version by one

* ## This is pre-alpha version. Don't rely on it, there could be and will be many bugs/vulnerabilities during all the pre-alpha and beta tests. You use it "as-is", I can't guarantee anything.

## For developers

### This project is currently under development. You are welcome if you want to participate in the process of creating bridge between max and telegram.

P.S. I'm working on this project alone, so I can't publish major updates really ofter, but I'll try my best.