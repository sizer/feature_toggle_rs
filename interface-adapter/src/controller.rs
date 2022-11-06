use app::UseCase;
use domain::{MyResult, Repositories};

use crate::{
    AddFeatureRequestDTO, AddFeatureResponseDTO, AddUserRequestDTO, AddUserResponseDTO,
    GetFeatureRequestDTO, GetFeatureResponseDTO, SearchFeatureRequestDTO, SearchFeatureResponseDTO,
    SearchUserRequestDTO, SearchUserResponseDTO, UpdateUserRequestDTO, UpdateUserResponseDTO,
};

pub mod dto;

pub struct Controller<'r, R: Repositories> {
    use_case: UseCase<'r, R>,
}

impl<'r, R: Repositories> Controller<'r, R> {
    pub fn new(repositories: &'r R) -> Self {
        let use_case = UseCase::new(repositories);
        Self { use_case }
    }

    pub fn search_users(&self, dto: SearchUserRequestDTO) -> SearchUserResponseDTO {
        let users = self.use_case.search_users(
            dto.first_name.as_ref(),
            dto.last_name.as_ref(),
            dto.email.as_ref(),
        );
        SearchUserResponseDTO { users }
    }

    pub fn add_user(&self, dto: AddUserRequestDTO) -> MyResult<AddUserResponseDTO> {
        let user = dto.user;
        self.use_case.add_user(user).map(|()| AddUserResponseDTO {})
    }

    pub fn update_user(&self, dto: UpdateUserRequestDTO) -> MyResult<UpdateUserResponseDTO> {
        self.use_case
            .update_user_by_email(&dto.email, dto.first_name, dto.last_name)
            .map(|()| UpdateUserResponseDTO {})
    }

    pub fn search_features(&self, dto: SearchFeatureRequestDTO) -> SearchFeatureResponseDTO {
        let features = self.use_case.search_features(dto.name.as_ref());
        SearchFeatureResponseDTO { features }
    }

    pub fn add_feature(&self, dto: AddFeatureRequestDTO) -> MyResult<AddFeatureResponseDTO> {
        self.use_case
            .add_feature(dto.feature)
            .map(|()| AddFeatureResponseDTO {})
    }

    pub fn get_feature(&self, dto: GetFeatureRequestDTO) -> MyResult<GetFeatureResponseDTO> {
        self.use_case
            .get_features(dto.user_id)
            .map(|f| GetFeatureResponseDTO { features: f })
    }
}
