#!/bin/bash

#Script file for deploy naive fortunes

FORT_URL="https://gist.githubusercontent.com/FiveYellowMice/c0581783983b05ca28f7/raw/7217e8b658aa0fe058e1ea742ede612040ed2a37/fortunes.json"

if test -e ./src ;then
    echo "I: src dir found,invoking cargo to build source..."
    command cargo 2> /dev/null
    if [ $? = 127 ] ;then
        echo "cargo not found,install latest nightly rust toolchain first."
        exit 1
    else
        cargo install --force
        echo "cargo exited with "$?
        echo "grabbing fortunes from FiveYellowMice's gist..."
        command wget 2> /dev/null
        if [ $? = 127 ] ;then
            echo "please install wget first ..."
            exit 1
        else
            wget -q $FORT_URL -O ~/fort.json
        fi 
        echo "wget exited with"$?
        echo "preparing cron... (root access needed) (PLEASE INSTALL cronie !!!)"
        read -p "please install cronie now, or press Ctrl+C"
        sudo echo "#!/bin/sh" > /etc/cron.daily/fortnaive
        sudo echo "wget "$FORT_URL" -O "${HOME}/fort.json >> /etc/cron.daily/fortnaive
        sudo chmod +x /etc/cron.daily/fortnaive
        sudo echo "24 * * * * root systemctl restart fort.service" >> /etc/cron.d/0daily
        sudo echo "24 * * * * root run-parts /etc/cron.daily" >> /etc/cron.d/0daily
        sudo crontab /etc/cron.d/0daily
        cfgfile=`cat /etc/cron.daily/fortnaive`
        echo "${cfgfile} writed"
        echo "preparing systemd service (this will panic when running on non-systemd distro)"
        echo "(root access needed)"
        echo "\[Unit]
Description=fortune
After=network.target

[Service]
ExecStart=${HOME}/.cargo/bin/naivefortunes ${HOME}/fort.json
Restart=always
RestartSec=0
Environment=ROCKET_ENV=production" > /lib/systemd/system/fort.service
       echo "done. use systemctl start fort.service to start."
    fi
else
    echo "please run this script in project dir."
    exit 1
fi
