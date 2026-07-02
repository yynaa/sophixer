local massage = require("massage")

massage.build_schema(massage.schema_from_path("../../schema/from_renoise.toml"), "../xyz.yyna.Calcium.xrnx/messages", "messages")
massage.build_schema(massage.schema_from_path("../../schema/to_renoise.toml"), "../xyz.yyna.Calcium.xrnx/messages", "messages")
