// Copyright 2021 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::BatchStoreOperations;
use crate::batches::store::{diesel::schema::batches, BatchStoreError};
use crate::error::InternalError;

use diesel::{dsl::update, prelude::*};

pub(in crate::batches::store::diesel) trait ChangeBatchToSubmittedOperation {
    fn change_batch_to_submitted(&self, id: &str) -> Result<(), BatchStoreError>;
}

#[cfg(feature = "postgres")]
impl<'a> ChangeBatchToSubmittedOperation for BatchStoreOperations<'a, diesel::pg::PgConnection> {
    fn change_batch_to_submitted(&self, id: &str) -> Result<(), BatchStoreError> {
        update(batches::table)
            .filter(batches::header_signature.eq(id))
            .set(batches::submitted.eq(true))
            .execute(self.conn)
            .map(|_| ())
            .map_err(|err| {
                BatchStoreError::InternalError(InternalError::from_source(Box::new(err)))
            })
    }
}

#[cfg(feature = "sqlite")]
impl<'a> ChangeBatchToSubmittedOperation
    for BatchStoreOperations<'a, diesel::sqlite::SqliteConnection>
{
    fn change_batch_to_submitted(&self, id: &str) -> Result<(), BatchStoreError> {
        update(batches::table)
            .filter(batches::header_signature.eq(id))
            .set(batches::submitted.eq(true))
            .execute(self.conn)
            .map(|_| ())
            .map_err(|err| {
                BatchStoreError::InternalError(InternalError::from_source(Box::new(err)))
            })
    }
}
