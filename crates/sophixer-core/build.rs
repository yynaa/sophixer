fn main() {
  massage::generate::build_schema(
    massage::format::Schema::from_path("../../schema/from_renoise.toml").unwrap(),
  );
  massage::generate::build_schema(
    massage::format::Schema::from_path("../../schema/to_renoise.toml").unwrap(),
  );
}
