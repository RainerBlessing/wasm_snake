FROM ubuntu:16.04

RUN apt update
RUN apt install build-essential curl -y
RUN apt install file -y
RUN apt install asciinema -y
RUN apt install unzip -y
RUN apt install git -y
RUN apt install libssl-dev -y
RUN apt install zlib1g-dev -y
RUN apt install gnupg -y
RUN apt install pkg-config -y

RUN useradd -ms /bin/bash rust
RUN curl -sL https://deb.nodesource.com/setup_10.x  | bash -
RUN apt-get -y install nodejs

#USER rust
#ENV HOME /home/rust
#ENV USER rust
#ENV SHELL /bin/bash
#WORKDIR /home/rust

RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y
RUN echo "export PATH=~/.cargo/bin:$PATH" >> ~/.bashrc
RUN echo "export PS1='\u:\w$ '" >> ~/.bashrc

# Create app directory
WORKDIR /usr/src/app

# Install app dependencies
# A wildcard is used to ensure both package.json AND package-lock.json are copied
# where available (npm@5+)
COPY package*.json ./
ENV NODE_ENV=development
ENV PORT=8080
ENV mode=development
RUN npm install
# If you are building your code for production
# RUN npm ci --only=production

# Bundle app source
COPY . .
#RUN npm run build

EXPOSE 8080
ENV PATH="/root/.cargo/bin:${PATH}"
CMD [ "npm", "run","serve" ]
