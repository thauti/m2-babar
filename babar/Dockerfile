FROM rust:1.23.0

WORKDIR /ws/
COPY . .
RUN cargo install

CMD ["ws","ressources/data.csv", "ressources/user.txt", "ressources/view0.csv", "ressources/changes.txt"]
