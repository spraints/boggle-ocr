FROM ruby:3.0.1

WORKDIR /app
COPY Gemfile /app
COPY Gemfile.lock /app
RUN bundle install
COPY . /app
CMD ["bin/rails", "server"]
