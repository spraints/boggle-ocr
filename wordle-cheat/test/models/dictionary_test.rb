require "test_helper"

class DictionaryTest < ActiveSupport::TestCase
  test "parse dictionary" do
    dict = Dictionary.parse(Rails.root.join("config/dictionary"))
    assert dict.has_word?("cat"), "dictionary should have the word 'cat'"
    refute dict.has_word?("fhqwhgads"), "dictionary should have the word 'fhqwhgads'"
  end
end
