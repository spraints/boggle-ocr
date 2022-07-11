require "test_helper"

class GuesstimatorTest < ActiveSupport::TestCase
  def guess(*guesses)
    Guesstimator.guess(guesses.map { |g| Guess.new(g) })
  end

  # http://127.0.0.1:3000/?previous_guess%5B%5D=BL%5BI%5DND&previous_guess%5B%5D=%28C%29H%5BI%5DPS&guess=tr%5Bic%5Dk
  test "voice" do
    assert_includes guess("bl[i]nd", "(c)h[i]ps", "tr[ic]k"), "voice", "guesses for bl[i]nd,(c)h[i]ps,tr[ic]k"
  end

  # http://127.0.0.1:3000/?previous_guess%5B%5D=G%28R%29OUP&previous_guess%5B%5D=%28R%29AILS&guess=%28th%29%5Br%5D%28e%29e
  test "berth" do
    assert_equal ["berth", "hertz"], guess("g(r)oup", "(r)ails", "(th)[r](e)e"), "guesses for g(r)oup,(r)ails,(th)[r](e)e"
  end

  # http://127.0.0.1:3000/?previous_guess%5B%5D=TRIE%28D%29&previous_guess%5B%5D=%28D%29%5BA%5DUBS&guess=c%5Ba%5Dn%28d%29y
  test "madam" do
    assert_equal ["gadjo", "hadal", "madam"], guess("trie(d)", "(d)[a]ubs", "c[a]n(d)y"), "guesses for trie(d),(d)[a]ubs,c[a]n(d)y"
  end
end
