from .base import Base
import glob
import itertools
import os


class Source(Base):
    def __init__(self, vim):
        super().__init__(vim)

        self.name = 'sample'
        self.kind = 'sample'

    def on_init(self, context):
        print('on_init')

    def on_close(self, context):
        print('on_close')

    def gather_candidates(self, context):
        return [
                {'word': 'first'},
                {'word': 'second'},
                {'word': 'third'},
                ]
