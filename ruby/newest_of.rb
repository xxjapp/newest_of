#!/usr/bin/env ruby
# encoding: utf-8
#
# Introduction:
#   get newest modified time of files in a folder
#
# Examples:
#   > ruby newest_of.rb ./
#   > ruby newest_of.rb /tmp
#

require 'find'

################################################################
# methods

module Test
    def self.newest_of dir
        newest = 0

        Find.find dir do |path|
            newest = [newest, File.mtime(path).to_i].max
        end

        newest
    end
end

################################################################
# main

if __FILE__ == $0
    puts Test.newest_of ARGV[0]
end
