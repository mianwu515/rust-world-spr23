#!/bin/bash


echo "First we are going to install some command line tools for Mac..."
echo "You will be asked for your password to run 'sudo' on this.  "
echo "If you get an error about the tools already being installed, just ignore it"
# install command line tools
sudo xcode-select --install


# install brew
x=`which brew`
if [ "$x" == "" ]
then
    echo "Installing brew (you will probably need to type your password again)"
    
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"
else
    echo "It appears you already have brew installed. skipping that step"
fi
echo "Installing some packages"
# Install emacs and java.
#sudo apt install emacs openjdk-14-jdk clang-format b
brew install java
brew install emacs
brew install clang-format
brew install wget
sudo ln -sfn /usr/local/opt/openjdk/libexec/openjdk.jdk /Library/Java/JavaVirtualMachines/openjdk.jdk



# lets see what there is for gradle
x=`which gradle`
if [ "$x" != "" ]
then
    echo "You seem to already have gradle installed.."
    v=`gradle --version | grep Gradle`
    echo "The version is $v"
    vnum=`echo $v | cut -f2 -d" "`
    major=`echo $vnum | cut -f1 -d"."`
    minor=`echo $vnum | cut -f2 -d"."`
    if [ "$major" != "7" ]
    then 
	echo "This has only been tested with gradle 7.x"
	echo "We don't really know what will happen with this version."
	echo "Either switch to gradle 7.x (recommended 7.3.3) or "
	echo "If you are really sure, remove the exit statement after this line in the script..."
	exit 1
    elif [ "$minor" != "3" ]
    then
	echo "This version is probably fine, but we have tested with 7.3.3"
    fi
else
    echo "Lets install gradle"
    echo "Downloading gradle 7.3"
    GRADLE_VERSION=7.3.3
    wget https://services.gradle.org/distributions/gradle-${GRADLE_VERSION}-bin.zip
    sudo mkdir -p /opt/local
    sudo unzip -d /opt/local gradle-${GRADLE_VERSION}-bin.zip
    sudo ln -s /opt/local/gradle-${GRADLE_VERSION}/bin/gradle /usr/local/bin/gradle
fi

brew install gradle-completion
