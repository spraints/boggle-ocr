require "test_helper"

class GuesstimatorTest < ActiveSupport::TestCase
  def guess(*guesses)
    Guesstimator.guess(guesses.map { |g| Guess.new(g) })
  end

  test "voice" do
    assert_includes guess("bl[i]nd", "(c)h[i]ps", "tr[ic]k"), "voice", "guesses for bl[i]nd,(c)h[i]ps,tr[ic]k"
  end
end
