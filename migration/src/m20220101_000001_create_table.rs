use sea_orm_migration::prelude::*;

// #[derive(DeriveMigrationName)]
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Paste::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Paste::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Paste::Filename).string().not_null())
                    .col(ColumnDef::new(Paste::Content).string().not_null())
                    .col(ColumnDef::new(Paste::CreateTime).date_time().not_null())
                    .col(ColumnDef::new(Paste::ExpireAfter).big_integer())
                    .col(ColumnDef::new(Paste::ExpireTime).date_time())
                    .col(
                        ColumnDef::new(Paste::Private)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Paste::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#ide
#[derive(Iden)]
enum Paste {
    Table,
    Id,
    Filename,
    Content,
    CreateTime,
    ExpireAfter,
    ExpireTime,
    Private,
}
