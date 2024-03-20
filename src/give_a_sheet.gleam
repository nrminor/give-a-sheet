import argv
import simplifile.{append, get_files, verify_is_directory, write}
import gleam/list.{filter, first, length, map, unique}
import gleam/io
import gleam/result
import gleam/string

pub type IlluminaLine {
  Fields(sample: String, fastq_1: String, fastq_2: String)
}

pub type Arguments {
  Args(input_dir: String, output_prefix: String)
}

fn parse_command_line_args() -> Result(Arguments, String) {
  case argv.load().arguments {
    [input_dir, output_prefix] -> {
      Ok(Args(string.trim(input_dir), output_prefix))
    }
    _ -> {
      Error("usage: ./give_a_sheet <path/to/directory> <output_prefix>")
    }
  }
}

fn collect_fields(sample_id: String, fastq_files: List(String)) -> IlluminaLine {
  let fastqs =
    fastq_files
    |> filter(fn(x) { string.contains(x, sample_id) })

  let fastq1 =
    fastqs
    |> filter(fn(x) { string.contains(x, "R1") })
    |> first
    |> result.unwrap("")

  let fastq2 =
    fastqs
    |> filter(fn(x) { string.contains(x, "R2") })
    |> first
    |> result.unwrap("")

  Fields(sample_id, fastq1, fastq2)
}

fn retrieve_sample_ids(fastq_files: List(String)) -> List(String) {
  fastq_files
  |> map(fn(x) { string.replace(x, ".fastq.gz", "") })
  |> map(fn(x) { string.replace(x, "_L001_R1_001", "") })
  |> map(fn(x) { string.replace(x, "_L001_R2_001", "") })
  |> unique
  |> map(fn(x) { string.split(x, "/") })
  |> map(fn(x) {
    let basename_attempt = list.last(x)
    case basename_attempt {
      Ok(sample_id) -> sample_id
      Error(_) -> {
        io.debug(
          "Error encountered while parsing sample IDs for the following path:\n"
          <> string.join(x, with: "/"),
        )
        ""
      }
    }
  })
  |> filter(fn(x) { x != "" })
}

fn write_csv_lines(
  lines: List(String),
  output_name: String,
) -> Result(Nil, String) {
  // write out the header
  let header_result =
    "sample,fastq_1,fastq_2\n"
    |> write(to: output_name)
  case header_result {
    Error(_) -> {
      io.println("Failed to write header to " <> output_name)
      io.println(
        "Check file permissions to make sure a file can be written in the current directory.",
      )
    }
    _ -> Nil
  }

  // write out the lines
  let line_write_results =
    lines
    |> map(fn(x) { append(x, to: output_name) })

  // log writing errors for any of the lines
  line_write_results
  |> map(fn(x) {
    case x {
      Error(write_error) -> {
        io.println("Failed to write lines because of the following error(s):")
        io.debug(write_error)
        Nil
      }
      _ -> Nil
    }
  })

  Ok(Nil)
}

pub fn main() {
  // pull in command line arguments and parse them if found
  let args_result = parse_command_line_args()
  let assert Ok(args) = args_result
  let input_dir = args.input_dir
  let output_prefix = args.output_prefix

  // name output file
  let output_file = output_prefix <> ".csv"

  // make sure the directory exists
  let assert Ok(True) = verify_is_directory(input_dir)

  // list files and sort out the FASTQs
  let file_result = get_files(input_dir)

  // use pattern matching to unwrap and filter the files to just FASTQs
  let fastq_files: List(String) = case file_result {
    Ok(files) ->
      files
      |> filter(fn(x) { string.contains(x, "fastq.gz") })
    Error(_) -> []
  }

  // abort if there are no fastqs
  let file_count = length(fastq_files)
  case file_count {
    file_count if file_count == 0 -> {
      panic as "No FASTQ files found in" <> input_dir
      Nil
    }
    _ -> Nil
  }

  // make a new list of all unique sample IDs
  let sample_ids = retrieve_sample_ids(fastq_files)

  // collect a list of illumina lines
  let lines: List(String) =
    sample_ids
    |> map(fn(x) { collect_fields(x, fastq_files) })
    |> map(fn(x) { x.sample <> "," <> x.fastq_1 <> "," <> x.fastq_2 <> "\n" })

  // write out the file
  write_csv_lines(lines, output_file)
}
