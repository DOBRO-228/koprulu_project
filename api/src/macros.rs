#[macro_export]
macro_rules! generate_base_crud_handlers {
    ($handler_prefix:ident, $entity:ident, $model:ident, $input:ident, $active_model:ident, $column_for_sorting:expr) => {
        paste::paste! {
            use axum::extract::Path;
            use sea_orm::{DatabaseConnection, ActiveModelTrait};

            // Handler to get all entities
            pub async fn [<get_all_ $handler_prefix s>](
                State(state): State<AppState>,
            ) -> Result<(StatusCode, Json<Vec<$model>>), AppError> {
                let db: &DatabaseConnection = &state.db;

                let items = $entity::find()
                    .order_by_asc($column_for_sorting)
                    .all(db)
                    .await
                    .map_err(AppError::from)?;

                Ok((StatusCode::OK, Json(items)))
            }

            // Handler to get a single entity by ID
            pub async fn [<get_ $handler_prefix>](
                State(state): State<AppState>,
                Path(id): Path<i32>,
            ) -> Result<(StatusCode, Json<$model>), AppError> {
                let db: &DatabaseConnection = &state.db;

                let item = $entity::find_by_id(id)
                    .one(db)
                    .await
                    .map_err(AppError::from)?;

                if let Some(item) = item {
                    Ok((StatusCode::OK, Json(item)))
                } else {
                    Err(AppError::NotFound(format!(
                        "{} with id {} not found",
                        stringify!($model),
                        id
                    )))
                }
            }

            // Handler to create a new entity
            pub async fn [<create_ $handler_prefix>](
                State(state): State<AppState>,
                Json(input): Json<$input>,
            ) -> Result<(StatusCode, Json<$model>), AppError> {
                let db: &DatabaseConnection = &state.db;

                let active_model: $active_model = input.into_active_model();

                let item = active_model
                    .insert(db)
                    .await
                    .map_err(AppError::from)?;

                Ok((StatusCode::CREATED, Json(item)))
            }

            // Handler to update an existing entity
            pub async fn [<update_ $handler_prefix>](
                State(state): State<AppState>,
                Path(id): Path<i32>,
                Json(input): Json<$input>,
            ) -> Result<(StatusCode, Json<$model>), AppError> {
                let db: &DatabaseConnection = &state.db;

                let mut active_model: $active_model = input.into_active_model();
                active_model.id = Set(id);

                let item = active_model
                    .update(db)
                    .await
                    .map_err(|e| match e {
                        sea_orm::DbErr::RecordNotFound(msg) => AppError::NotFound(msg),
                        other_err => AppError::from(other_err),
                    })?;

                Ok((StatusCode::OK, Json(item)))
            }

            // Handler to delete an entity
            pub async fn [<delete_ $handler_prefix>](
                State(state): State<AppState>,
                Path(id): Path<i32>,
            ) -> Result<StatusCode, AppError> {
                let db: &DatabaseConnection = &state.db;

                let result = $entity::delete_by_id(id)
                    .exec(db)
                    .await
                    .map_err(AppError::from)?;

                if result.rows_affected == 0 {
                    Err(AppError::NotFound(format!(
                        "{} with id {} not found",
                        stringify!($model),
                        id
                    )))
                } else {
                    Ok(StatusCode::NO_CONTENT)
                }
            }
        }
    };
}
