
# Reminder calendar

![Preview of website](/preview.jpg)

To run the container use:
`sudo docker run -p 8000:8000 -p 12137:12137 -v ./data.json:/reminder-calendar/website/data.json -v ./consts.rs:/reminder-calendar/website/src/consts.rs infinitycity/reminder-calendar:latest`

data.json holds reminder data and name of the months, if you wish to change them.
The consts.rs file holds server address for trunk, this is the address that is accessed by browser to fetch data, in container, this is port 12137, the port for accessing the website is by default 8000.

If you wish to build the container yourself:
```
git clone https://github.com/InfinityCity18/reminder-calendar.git (or just download Dockerfile)
docker build -t reminder-calendar .
sudo docker run -p 8000:8000 -p 12137:12137 -v ./data.json:/reminder-calendar/website/data.json -v ./consts.rs:/reminder-calendar/website/src/consts.rs reminder-calendar
```

I've also made iOS Shortcut for notifying of reminders:

iOS Shortcuts link : https://www.icloud.com/shortcuts/46e252b180204331b27e7d9711d8e9f9
