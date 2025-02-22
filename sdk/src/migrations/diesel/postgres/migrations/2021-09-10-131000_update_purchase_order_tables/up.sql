-- Copyright 2021 Cargill Incorporated
--
-- Licensed under the Apache License, Version 2.0 (the "License");
-- you may not use this file except in compliance with the License.
-- You may obtain a copy of the License at
--
--     http://www.apache.org/licenses/LICENSE-2.0
--
-- Unless required by applicable law or agreed to in writing, software
-- distributed under the License is distributed on an "AS IS" BASIS,
-- WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
-- See the License for the specific language governing permissions and
-- limitations under the License.
-- -----------------------------------------------------------------------------

ALTER TABLE purchase_order RENAME COLUMN uuid TO purchase_order_uid;
ALTER TABLE purchase_order DROP COLUMN org_id;
ALTER TABLE purchase_order ADD COLUMN buyer_org_id VARCHAR(256) NOT NULL;
ALTER TABLE purchase_order ADD COLUMN seller_org_id VARCHAR(256) NOT NULL;

ALTER TABLE purchase_order_version RENAME COLUMN purchase_order_uuid TO purchase_order_uid;
ALTER TABLE purchase_order_version DROP COLUMN org_id;

ALTER TABLE purchase_order_version_revision DROP COLUMN org_id;

ALTER TABLE purchase_order_alternate_id RENAME COLUMN purchase_order_uuid TO purchase_order_uid;
