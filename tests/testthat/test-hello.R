test_that("Call to Rust function `hello_world()` works", {
  expect_equal(hello_world(), "Hello world!")
})

test_that("Calling stuffs returns successfully", {
  source <- Source$new("blah")
  expect_equal(source$path(), "blah")
  expect_equal(source$stuffs(), c("THING"))
})
