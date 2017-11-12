INSERT INTO "session" DEFAULT VALUES ;
SELECT create_user(1, 'd@danilo.o', 'password*$#@', 'Danilo', 'Pereira');
SELECT create_user(1, 'v@let.c', 'senha123@#$%', 'Viviane', 'Almeida');
UPDATE "user" SET external_id = 'f47701f7-cabc-4db2-b2fc-053c9137c447' WHERE id =1;
UPDATE "user" SET external_id = '7f9eac7e-f07c-4556-9539-c95ceab5c97f' WHERE id =2;