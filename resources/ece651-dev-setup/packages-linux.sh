#!/bin/bash

echo "Before I start, should I update your network settings to use Google's DNS? "
echo " - If you are running in VirtualBox, this is likely a good idea"
echo " - If you are running somewhere else, it is probably fine as long as you have root"
echo " Enter y or n"
read donet
if [ "$donet" == "y" -o "$donet" == "Y" ]
then
    echo "Updating networking settings..."
    echo " - Making a backup of /etc/systemd/resolv.conf locally"
    cp /etc/systemd/resolved.conf ./
    cat > newresolved.conf <<EOF
#  This file is part of systemd.
#
#  systemd is free software; you can redistribute it and/or modify it
#  under the terms of the GNU Lesser General Public License as published by
#  the Free Software Foundation; either version 2.1 of the License, or
#  (at your option) any later version.
#
# Entries in this file show the compile time defaults.
# You can change settings by editing this file.
# Defaults can be restored by simply deleting this file.
#
# See resolved.conf(5) for details

[Resolve]
DNS=8.8.8.8 8.8.4.4
#FallbackDNS=
Domains=~.
#LLMNR=no
#MulticastDNS=no
#DNSSEC=no
#DNSOverTLS=no
#Cache=yes
#DNSStubListener=yes
#ReadEtcHosts=yes

EOF
    sudo cp newresolved.conf /etc/systemd/resolved.conf
    sudo service systemd-resolved restart
else
    echo "Skipping network settings"
fi


# Install emacs and java.
sudo apt install emacs unzip openjdk-17-jdk clang-format 

#git config --global credential.helper store
#We want 7.3.3 in particular.
GRADLE_VERSION=7.3.3

wget https://services.gradle.org/distributions/gradle-${GRADLE_VERSION}-bin.zip
sudo mkdir -p /opt/gradle
sudo unzip -d /opt/gradle gradle-${GRADLE_VERSION}-bin.zip
rm gradle-${GRADLE_VERSION}-bin.zip
#for this time
export PATH=${PATH}:/opt/gradle/${GRADLE_VERSION}/bin
#for the future
echo "export PATH=\${PATH}:/opt/gradle/gradle-${GRADLE_VERSION}/bin" >> ~/.bashrc
# Add extra auto complete support to base for gradle commands
sudo apt install gradle-completion

echo "XTerm.termName: xterm-256color" >> ~/.Xresources
echo "xterm*faceName: Deja Sans Mono Book" >> ~/.Xresources
echo "xterm*faceSize: 18" >> ~/.Xresources
echo "EDITOR='emacs -nw'" >> ~/.bashrc 
echo "VISUAL=emacs" >> ~/.bashrc 

