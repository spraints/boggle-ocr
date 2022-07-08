require "test_helper"

class GuessTest < ActiveSupport::TestCase
  test "parse abcde" do
    g = Guess.new("abcde")

    assert_equal [
      ["A", Guess::NOT_IN_PUZZLE],
      ["B", Guess::NOT_IN_PUZZLE],
      ["C", Guess::NOT_IN_PUZZLE],
      ["D", Guess::NOT_IN_PUZZLE],
      ["E", Guess::NOT_IN_PUZZLE],
    ], g.letters

    assert_equal "ABCDE", g.to_param
  end

  test "parse a[bc](de)" do
    g = Guess.new("a[bc](de)")

    assert_equal [
      ["A", Guess::NOT_IN_PUZZLE],
      ["B", Guess::CORRECT_POSITION],
      ["C", Guess::CORRECT_POSITION],
      ["D", Guess::INCORRECT_POSITION],
      ["E", Guess::INCORRECT_POSITION],
    ], g.letters

    assert_equal "A[B][C](D)(E)", g.to_param
  end

  test "alter letter" do
    g = Guess.new("abc(d)e")
    h = g.with(3, Guess::NOT_IN_PUZZLE)

    assert_equal "ABCDE", h.to_param, "abc(d)e with(3, not-in-puzzle)"
    assert_equal "ABC(D)E", g.to_param, "original object should not be changed"
  end
end
