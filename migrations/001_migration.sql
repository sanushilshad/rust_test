
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE user_type AS ENUM (
  'guest',
  'user',
  'member',
  'agent',
  'superadmin',
  'admin'
);

CREATE TYPE status AS ENUM (
  'active',
  'inactive',
  'pending',
  'archived'
);

CREATE TABLE IF NOT EXISTS user_account(
    id uuid PRIMARY KEY,
    is_test_user BOOLEAN NOT NULL DEFAULT false,
    username TEXT NOT NULL UNIQUE,
    international_dialing_code TEXT NOT NULL,
    mobile_no TEXT NOT NULL,
    source data_source NOT NULL,
    email TEXT NOT NULL,
    display_name TEXT NOT NULL,
    user_account_number TEXT NOT NULL,
    alt_user_account_number TEXT NOT NULL,
    is_active status DEFAULT 'active'::status,
    created_by uuid NOT NULL,
    vectors jsonb NOT NULL,
    updated_by uuid,
    deleted_by uuid,
    created_on TIMESTAMPTZ NOT NULL,
    updated_on TIMESTAMPTZ,
    deleted_on TIMESTAMPTZ,
    is_deleted BOOLEAN NOT NULL DEFAULT false,
);

CREATE TYPE "user_auth_identifier_scope" AS ENUM (
  'otp',
  'password',
  'google',
  'facebook',
  'microsoft',
  'apple',
  'token',
  'auth_app',
  'qr',
  'email'
);

CREATE TYPE auth_context_type AS ENUM (
  'user_account',
  'business_account'
)

CREATE TABLE IF NOT EXISTS auth_mechanism (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL,
  auth_scope user_auth_identifier_scope NOT NULL,
  auth_identifier text NOT NULL,
  auth_context auth_context_type NOT NULL, 
  secret TEXT,
  valid_upto TIMESTAMPTZ,
  is_active status DEFAULT 'active'::status NOT NULL,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ,
  deleted_at TIMESTAMPTZ,
  created_by TEXT,
  updated_by TEXT,
  deleted_by TEXT,
  is_deleted BOOLEAN DEFAULT false
);

ALTER TABLE auth_mechanism ADD CONSTRAINT fk_auth_user FOREIGN KEY (user_id) REFERENCES user_account (id) ON DELETE CASCADE;
ALTER TABLE auth_mechanism ADD CONSTRAINT fk_auth_user_id_auth_scope UNIQUE (user_id, auth_scope, auth_context);



CREATE TYPE customer_type AS ENUM (
  'na',
  'buyer',
  'seller',
  'brand',
  'logistic_partner',
  'payment_aggregator',
  'virtual_operator',
  'external_partner'
);



CREATE TABLE IF NOT EXISTS role (
  id uuid PRIMARY KEY,
  role_name TEXT NOT NULL,
  role_status status  NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ,
  deleted_at TIMESTAMPTZ,
  created_by uuid NOT NULL,
  updated_by uuid,
  deleted_by uuid,
  is_deleted BOOLEAN  NOT NULL DEFAULT false
);

ALTER TABLE role ADD CONSTRAINT unique_role_name UNIQUE (role_name);

CREATE TABLE IF NOT EXISTS user_role (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL ON DELETE CASCADE,
  role_id uuid NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ,
  deleted_at TIMESTAMPTZ,
  created_by uuid NOT NULL,
  updated_by uuid,
  deleted_by uuid,
  is_deleted BOOLEAN NOT NULL
);

ALTER TABLE user_role ADD CONSTRAINT fk_role_id FOREIGN KEY ("role_id") REFERENCES role ("id") ON DELETE CASCADE;
ALTER TABLE user_role ADD CONSTRAINT fk_user_id FOREIGN KEY ("user_id") REFERENCES user_account ("id") ON DELETE CASCADE;
ALTER TABLE user_role ADD CONSTRAINT user_role_pk UNIQUE (user_id, role_id);

CREATE TABLE IF NOT EXISTS permission (
  id uuid PRIMARY KEY,
  permission_name TEXT,
  permission_description TEXT,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ,
  deleted_at TIMESTAMPTZ,
  created_by uuid,
  updated_by uuid,
  deleted_by uuid,
  is_deleted BOOLEAN
);

CREATE TABLE IF NOT EXISTS role_permission (
  id uuid PRIMARY KEY,
  role_id uuid,
  permission_id uuid,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ,
  deleted_at TIMESTAMPTZ,
  created_by uuid,
  updated_by uuid,
  deleted_by uuid,
  is_deleted BOOLEAN
);


ALTER TABLE role_permission ADD CONSTRAINT "fk_permission_id" FOREIGN KEY ("permission_id") REFERENCES permission ("id") ON DELETE CASCADE;
ALTER TABLE role_permission ADD CONSTRAINT "fk_role_id" FOREIGN KEY ("role_id") REFERENCES role ("id") ON DELETE CASCADE;
ALTER TABLE permission ADD CONSTRAINT permission_name UNIQUE (permission_name);
ALTER TABLE role_permission ADD CONSTRAINT permission_role_id UNIQUE (permission_id, role_id);


CREATE TABLE IF NOT EXISTS communication (
  id uuid PRIMARY KEY,
  message TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  created_by TEXT NOT NULL,
  media_list TEXT[]
);

CREATE TYPE kyc_status AS ENUM (
  'pending',
  'on-hold',
  'rejected',
  'completed'
);

CREATE TYPE "data_source" AS ENUM (
  'placeorder',
  'ondc',
  'rapidor'
);

CREATE TYPE trade_type as ENUM (
  'domestic',
  'export'
);

CREATE TYPE merchant_type as ENUM (
  'fpo',
  'manufacturer',
  'restaurant',
  'grocery',
  'mall',
  'supermart',
  'store',
  'other'
);

CREATE TABLE IF NOT EXISTS business_account (
  id uuid PRIMARY KEY,
  business_account_number TEXT NOT NULL,
  alt_business_account_number TEXT NOT NULL,
  company_name TEXT NOT NULL,
  vectors jsonb NOT NULL,
  proofs jsonb NOT NULL,
  customer_type customer_type NOT NULL,
  merchant_type merchant_type NOT NULL,
  trade trade_type[],
  tags TEXT[],
  is_active status DEFAULT 'active'::status,
  source data_source NOT NULL,
  opening_time TIME,
  closing_time TIME,
  kyc_status kyc_status DEFAULT 'pending'::kyc_status,
  kyc_completed_by uuid,
  metadata_json jsonb,
  created_by  uuid NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_by uuid,
  updated_at TIMESTAMPTZ,
  deleted_by uuid,
  deleted_at TIMESTAMPTZ,
  is_test_account BOOLEAN NOT NULL DEFAULT false

);

CREATE TABLE IF NOT EXISTS business_user_relationship (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL,
  business_id uuid NOT NULL,
  role_id uuid NOT NULL,
  verified BOOLEAN DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ,
  deleted_at TIMESTAMPTZ,
  created_by uuid NOT NULL,
  updated_by uuid,
  deleted_by uuid,
  is_deleted BOOLEAN DEFAULT false
);


ALTER TABLE business_user_relationship ADD CONSTRAINT "fk_user_id" FOREIGN KEY ("user_id") REFERENCES user_account ("id") ON DELETE CASCADE;
ALTER TABLE business_user_relationship ADD CONSTRAINT "fk_business_id" FOREIGN KEY ("business_id") REFERENCES business_account ("id") ON DELETE CASCADE;
ALTER TABLE business_user_relationship ADD CONSTRAINT "fk_role_id" FOREIGN KEY ("role_id") REFERENCES role ("id") ON DELETE CASCADE;
ALTER TABLE business_user_relationship ADD CONSTRAINT user_business_role UNIQUE (user_id, business_id, role_id);