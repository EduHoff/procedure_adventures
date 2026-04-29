# procedure_adventures

## Primeira execução / Rebuild
```
docker compose up --build
```

## Iniciar
```
docker compose up -d
docker exec -it procedure_adventures_dev cargo run
```

## Encerrar
```
docker compose down
```

## Reconstruir imagem
```
docker compose build --no-cache
```