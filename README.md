### Build and extract executable
```
docker build -t gatodown:prod .
id=$(docker create gatodown:prod)
docker cp $id:/bin/gatodown - > gatodown.tar
docker rm -v $id
tar xvf gatodown.tar
```
